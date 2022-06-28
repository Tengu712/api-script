%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int g_curLine;
int g_curChar;
FILE *yyin;
int yylex();
int g_cntArg = 0;
int g_isCallExid = 0;
FILE *g_pDefs = NULL;
FILE *g_pTarget = NULL;
FILE *g_pHeader = NULL;
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
%token FUN ARGS LOGIC CALL
%token VOID PTR I8 I16 I32 I64 U8 U16 U32 U64 F32 F64
%token NULLPTR
%token<data> STR INT FLOAT ID
%token<exid> EXID
%type<data> data type

%%

program		: blocks EOFILE	{ return 0; }
blocks		:
			| block blocks
block		: function

function	: FUN type ID		{ fprintf(g_pTarget, "%s %s", $2, $3);	fprintf(g_pHeader, "%s %s", $2, $3); }
			  funbody
funbody		:					{ fprintf(g_pTarget, "() {}\n");		fprintf(g_pHeader, "();\n"); }
			| INDENT			{ fprintf(g_pTarget, "() {\n");			fprintf(g_pHeader, "();\n"); }
			  logic DEDENT		{ fprintf(g_pTarget, "}\n"); }
			| INDENT			{ fprintf(g_pTarget, "(");				fprintf(g_pHeader, "("); }
			  args				{ fprintf(g_pTarget, ") {\n");			fprintf(g_pHeader, ");\n"); }
			  logic DEDENT		{ fprintf(g_pTarget, "}\n"); }
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
logic		: LOGIC INDENT call DEDENT

call		: CALL type ID		{ fprintf(g_pTarget, "    %s", $3); }
			  callargs
			| CALL type EXID 	{ fprintf(g_pTarget, "    %s", $3.id); fprintf(g_pDefs, "%s %s", $2, $3.id); g_isCallExid = 1; }
			  callargs			{ g_isCallExid = 0; }
callargs	:					{ fprintf(g_pTarget, "();\n");	if (g_isCallExid == 1) fprintf(g_pDefs, "();\n"); }
			| INDENT			{ fprintf(g_pTarget, "(");		if (g_isCallExid == 1) fprintf(g_pDefs, "("); }
			  callarg DEDENT	{ fprintf(g_pTarget, ");\n");	if (g_isCallExid == 1) fprintf(g_pDefs, ");\n"); }
callarg		:					{ g_cntArg = 0; }
			|					{
									if (g_cntArg != 0) {
										fprintf(g_pTarget, ", ");
										if (g_isCallExid == 1)
											fprintf(g_pDefs, ", ");
									}
									++g_cntArg;
								}
			  type data			{ fprintf(g_pTarget, "%s", $3);	if (g_isCallExid == 1) fprintf(g_pDefs, "%s a%d", $2, g_cntArg); }
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

%%

int main(int num_args, char **args) {
	setbuf(stdout, NULL);
	if (num_args < 2) {
		fprintf(stderr, "\e[91m[Fatal error]\e[0m no input files.\n");
		return 1;
	}
	int opt_static_lib = 0;
	int opt_left_tmpfiles = 0;
	for (int i = 1; i < num_args; ++i) {
		if (args[i][0] != '-')
			continue;
		if (strcmp(args[i], "-d") == 0)
			opt_static_lib = 1;
		else if (strcmp(args[i], "-a") == 0)
			opt_left_tmpfiles = 1;
		else {
			fprintf(stderr, "\e[91m[Fatal error]\e[0m invalid option. : %s\n", args[i]);
			return 1;
		}
	}

	// Create output files
	g_pDefs = fopen("a_.h", "w");
	if (g_pDefs == NULL) {
		fprintf(stderr, "\e[91m[IO error]\e[0m a header for definition of outer functions not created.\n");
		return 1;
	}
	g_pTarget = fopen("a.c", "w");
	if (g_pTarget == NULL) {
		fprintf(stderr, "\e[91m[IO error]\e[0m main source not created.\n");
		return 1;
	}
	g_pHeader = fopen("a.h", "w");
	if (g_pHeader == NULL) {
		fprintf(stderr, "\e[91m[IO error]\e[0m main header not created.\n");
		return 1;
	}
	fprintf(g_pTarget, "#include \"a_.h\"\n");
	fprintf(g_pHeader, "#pragma once\n");
	fprintf(g_pDefs, "#pragma once\n");

	// Parsing and printing
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

	// Comile based on compiler options
	int res = 1;
	if (opt_static_lib == 1) {
		res = system("gcc -c a.c");
		if (res == 0)
			res = system("ar rcs a.a a.o");
	} else {
		res = system("gcc a.c");
	}

	// Remove tmp file
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
	return 0;
}
