%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
extern int g_curLine;
extern int g_curChar;
extern int g_indentWidth;
extern FILE *yyin;
extern int yylex();
int g_cntArg = 0;
int g_isCallExid = 0;
int g_cntIndent = 0;
#define BUFSIZE 1204
char g_sBuf[BUFSIZE] = ""; 
FILE *g_pDefs = NULL;
FILE *g_pTarget = NULL;
FILE *g_pHeader = NULL;
typedef struct OLNArray_t {
	int num;
	int maxnum;
	char **libnames;
} OLNArray;
OLNArray *g_pOLNames = NULL;
void init_olnames() {
	char **p_tmp = (char**)malloc(sizeof(char*) * 4);
	memset(p_tmp, 0, sizeof(char*) * 4);
	g_pOLNames = (OLNArray*)malloc(sizeof(OLNArray*));
	g_pOLNames->num = 0;
	g_pOLNames->maxnum = 4;
	g_pOLNames->libnames = p_tmp;
}
void push_olname(char *libname) {
	if (g_pOLNames == NULL)
		init_olnames();
	if (g_pOLNames->num >= g_pOLNames->maxnum) {
		g_pOLNames->maxnum *= 2;
		char **p_tmp = (char**)malloc(sizeof(char*) * g_pOLNames->maxnum);
		memset(p_tmp, 0, sizeof(char*) * g_pOLNames->maxnum);
		memcpy(p_tmp, g_pOLNames->libnames, sizeof(char*) * g_pOLNames->num);
		free(g_pOLNames->libnames);
		g_pOLNames->libnames = p_tmp;
	}
	g_pOLNames->libnames[g_pOLNames->num] = libname;
	++g_pOLNames->num;
}
void indent() {
	++g_cntIndent;
}
void dedent() {
	--g_cntIndent;
}
void print_indent() {
	for (int i = 0; i < g_cntIndent; ++i) {
		fprintf(g_pTarget, "    ");
	}
}
void yyerror(char *msg) {
	fprintf(
		stderr,
		"\e[91m[Parsing error]\e[0m %s : %d line, %d char\n",
		msg,
		g_curLine,
		g_curChar
	);
}
%}

%code requires {
	typedef struct ExID_t {
		char* libname;
		char* id;
	} ExID;
}

%union {
	char* data;
	ExID exid;
}

%token EOFILE INDENT DEDENT
%token FUN ARGS LOGIC RETURN CALL IF ELIF ELSE
%token VOID PTR I8 I16 I32 I64 U8 U16 U32 U64 F32 F64
%token NULLPTR
%token<data> STR INT FLOAT ID INLINE
%token<exid> EXID
%type<data> data type call

%%

program		: blocks EOFILE		{ return 0; }
blocks		:
			| block blocks
block		: function

function	: FUN type ID		{ fprintf(g_pTarget, "%s %s", $2, $3);	fprintf(g_pHeader, "%s %s", $2, $3); }
			  funbody
funbody		:					{ fprintf(g_pTarget, "() {}\n");		fprintf(g_pHeader, "();\n"); }
			| INDENT			{ fprintf(g_pTarget, "() {\n");			fprintf(g_pHeader, "();\n");	indent(); }
			  logics DEDENT		{ fprintf(g_pTarget, "}\n");											dedent(); }
			| INDENT			{ fprintf(g_pTarget, "(");				fprintf(g_pHeader, "("); }
			  args				{ fprintf(g_pTarget, ") {\n");			fprintf(g_pHeader, ");\n");		indent(); }
			  logics DEDENT		{ fprintf(g_pTarget, "}\n");											dedent(); }
args		: ARGS INDENT funarg DEDENT
funarg		:					{ g_cntArg = 0; }
			|					{ 
									if (g_cntArg != 0) {
										fprintf(g_pTarget, ", ");
										fprintf(g_pHeader, ", ");
									}
									++g_cntArg;
								}
			  type ID			{ fprintf(g_pTarget, "%s %s", $2, $3);	fprintf(g_pHeader, "%s %s", $2, $3); }
			  funarg

logics		: LOGIC INDENT logic DEDENT
logic		:
			| return logic
			| let logic
			| if logic
			| call				{ print_indent(); fprintf(g_pTarget, "%s;\n", $1); }
			  logic
return		: RETURN data		{ print_indent(); fprintf(g_pTarget, "return %s;\n", $2); }
let			: type ID data		{ print_indent(); fprintf(g_pTarget, "%s %s = %s;\n", $1, $2, $3); }
if			: IF INLINE INDENT	{ print_indent(); indent(); fprintf(g_pTarget, "if (%s) {\n", $2); }
			  logic DEDENT		{ dedent(); print_indent(); fprintf(g_pTarget, "}\n"); }
			  ifex
ifex		:
			| ELSE INDENT		{ print_indent(); indent(); fprintf(g_pTarget, "else {\n");	}
			  logic DEDENT		{ dedent(); print_indent(); fprintf(g_pTarget, "}\n"); }
			| ELIF INLINE INDENT{ print_indent(); indent(); fprintf(g_pTarget, "else if (%s) {\n", $2);	}
			  logic DEDENT		{ dedent(); print_indent(); fprintf(g_pTarget, "}\n"); }
			  ifex
call		: CALL type ID		{
									memset(g_sBuf, 0, sizeof(char) * BUFSIZE);
									strcat(g_sBuf, $3);
								}
			  callargs			{
									$$ = strdup(g_sBuf);
			  					}
			| CALL type EXID	{
									g_isCallExid = 1;
									fprintf(g_pDefs, "%s %s", $2, $3.id);
									push_olname($3.libname);
									memset(g_sBuf, 0, sizeof(char) * BUFSIZE);
									strcat(g_sBuf, $3.id);
								}
			  callargs			{
									g_isCallExid = 0;
									$$ = strdup(g_sBuf);
								}
callargs	:					{
									if (g_isCallExid == 1)
										fprintf(g_pDefs, "();\n");
									strcat(g_sBuf, "()");
								}
			| INDENT			{
									if (g_isCallExid == 1)
										fprintf(g_pDefs, "(");
									strcat(g_sBuf, "(");
								}
			  callarg DEDENT	{
									if (g_isCallExid == 1)
										fprintf(g_pDefs, ");\n");
									strcat(g_sBuf, ")");
								}
callarg		:					{ g_cntArg = 0; }
			|					{
									if (g_cntArg != 0) {
										if (g_isCallExid == 1)
											fprintf(g_pDefs, ", ");
										strcat(g_sBuf, ", ");
									}
									++g_cntArg;
								}
			  type data			{
									if (g_isCallExid == 1)
										fprintf(g_pDefs, "%s a%d", $2, g_cntArg);
									char s_tmp[1024] = "";
									sprintf(s_tmp, "%s", $3);
									strcat(g_sBuf, s_tmp);
								}
			  callarg

type		: VOID		{ $$ = "void"; }
			| PTR		{ $$ = "void*"; }
			| I8		{ $$ = "char"; }
			| I16		{ $$ = "short"; }
			| I32		{ $$ = "int"; }
			| I64		{ $$ = "long long"; }
			| U8		{ $$ = "unsigned char"; }
			| U16		{ $$ = "unsigned short"; }
			| U32		{ $$ = "unsigned int"; }
			| U64		{ $$ = "unsigned long long"; }
			| F32		{ $$ = "float"; }
			| F64		{ $$ = "double"; }
data		: NULLPTR	{ $$ = "(void*)0"; }
			| STR | INT | FLOAT | ID
			| call

%%

void cat_olnames(char *command) {
	if (g_pOLNames == NULL)
		return;
	for (int i = 0; i < g_pOLNames->num; ++i) {
		strcat(command, "-l");
		strcat(command, g_pOLNames->libnames[i]);
		strcat(command, " ");
	}
}
int main(int num_args, char **args) {
	setbuf(stdout, NULL);
	if (num_args < 2) {
		fprintf(stderr, "\e[91m[Fatal error]\e[0m no input files.\n");
		return 1;
	}
	int opt_print = 0;
	int opt_not_build = 0;
	int opt_static_lib = 0;
	int opt_left_tmpfiles = 0;
	for (int i = 1; i < num_args; ++i) {
		if (args[i][0] != '-')
			continue;
		if (strcmp(args[i], "-p") == 0)
			opt_print = 1;
		else if (strcmp(args[i], "-c") == 0)
			opt_not_build = 1;
		else if (strcmp(args[i], "-d") == 0)
			opt_static_lib = 1;
		else if (strcmp(args[i], "-a") == 0)
			opt_left_tmpfiles = 1;
		else if (args[i][1] == 'i' && args[i][2] >= '1' && args[i][2] <= '9')
		    g_indentWidth = (int)args[i][2] - (int)'0';
		else {
			fprintf(stderr, "\e[91m[Fatal error]\e[0m invalid option. : %s\n", args[i]);
			return 1;
		}
	}

	// Create output files
	if (opt_print == 1) 
		printf("Creating temporary files\n");
	g_pDefs = fopen("a_.h", "w");
	if (g_pDefs == NULL) {
		fprintf(stderr, "\e[91m[IO error]\e[0m a header for definition of outer functions not created.\n");
		return 1;
	}
	if (opt_print == 1) 
		printf("    * a_.h\n");
	g_pTarget = fopen("a.c", "w");
	if (g_pTarget == NULL) {
		fprintf(stderr, "\e[91m[IO error]\e[0m main source not created.\n");
		return 1;
	}
	if (opt_print == 1) 
		printf("    * a.c\n");
	g_pHeader = fopen("a.h", "w");
	if (g_pHeader == NULL) {
		fprintf(stderr, "\e[91m[IO error]\e[0m main header not created.\n");
		return 1;
	}
	if (opt_print == 1) 
		printf("    * a.h\n");
	fprintf(g_pTarget, "#include \"a_.h\"\n");
	fprintf(g_pHeader, "#pragma once\n");
	fprintf(g_pDefs, "#pragma once\n");

	// Parsing and printing
	if (opt_print == 1) 
		printf("Parsing\n");
	int error = 0;
	for (int i = 1; i < num_args; ++i) {
		if (args[i][0] == '-')
			continue;
		FILE *p_file = fopen(args[i], "r");
		if (p_file == NULL) {
			fprintf(stderr, "\e[91m[IO error]\e[0m %s not opened.\n", args[i]);
			error = 1;
			break;
		}
		if (opt_print == 1) 
			printf("    * %s\n", args[i]);
		yyin = p_file;
		error = yyparse();
		fclose(p_file);
		if (error != 0)
			break;
	}
	fclose(g_pDefs);
	fclose(g_pTarget);
	fclose(g_pHeader);

	// If error remove all output file
	if (error != 0) {
#ifdef __linux__
		system("rm a_.h a.c a.h");
#elif _WIN32 || _WIN64
		system("del a_.h a.c a.h");
#endif
		return 1;
	}

	// If found -c option
	if (opt_not_build == 1)
		return 0;

	// Comile based on compiler options
	if (opt_print == 1) 
		printf("Compiling\n");
	char command[1024] = "";
	int res = 1;
	if (opt_static_lib == 1) {
		strcat(command, "gcc -c a.c ");
		cat_olnames(command);
		if (opt_print == 1) 
			printf("    * %s\n", command);
		res = system(command);
		if (res == 0) {
			if (opt_print == 1) 
				printf("    * ar rcs a.a a.o\n");
			res = system("ar rcs a.a a.o");
		}
	} else {
		strcat(command, "gcc a.c ");
		cat_olnames(command);
		if (opt_print == 1) 
			printf("    * %s\n", command);
		res = system(command);
	}

	// Remove tmp file
	if (opt_print == 1 && opt_left_tmpfiles == 0) 
		printf("Removing temporary files\n");
	if (opt_left_tmpfiles == 1) {
	} else if (opt_static_lib == 1) {
#ifdef __linux__
		system("rm a_.h a.c a.o");
#elif _WIN32 || _WIN64
		system("del a_.h a.c a.o");
#endif
	} else {
#ifdef __linux__
		system("rm a_.h a.c a.h");
#elif _WIN32 || _WIN64
		system("del a_.h a.c a.h");
#endif
	}
    if (res != 0) {
		fprintf(stderr, "\e[91m[IO error]\e[0m could not compiled.\n");
		return 1;
	}

	if (opt_print == 1) 
		printf("All processes succeeded\n");
	return 0;
}
