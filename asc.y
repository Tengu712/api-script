%{
#include <stdio.h>
#include <stdlib.h>
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
	exit(1);
}
%}

%token EOFILE INDENT DEDENT
%token FUN ARGS LOGIC CALL
%token VOID PTR I8 I16 I32 I64 U8 U16 U32 U64 F32 F64
%token NULLPTR STR INT FLOAT ID EXID 

%%

program		: blocks EOFILE	{ return 0; }
blocks		:
			| block blocks
block		: function

function	: FUN type			{ fprintf(g_pTarget, "type "); fprintf(g_pHeader, "type "); }
			  ID				{ fprintf(g_pTarget, "id"); fprintf(g_pHeader, "id"); }
			  funbody
funbody		: INDENT			{ fprintf(g_pTarget, "("); fprintf(g_pHeader, "("); }
			  args				{ fprintf(g_pTarget, ") {\n"); fprintf(g_pHeader, ");\n"); }
			  logic DEDENT		{ fprintf(g_pTarget, "}\n"); }
			| INDENT			{ fprintf(g_pTarget, "() {\n"); fprintf(g_pHeader, "();\n"); }
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
			  type ID			{ fprintf(g_pTarget, "type id"); fprintf(g_pHeader, "type id"); }
			  funarg
logic		: LOGIC INDENT call DEDENT

call		: CALL type ID		{ fprintf(g_pTarget, "    callid"); }
			  callargs
			| CALL type EXID 	{ fprintf(g_pTarget, "    callexid"); fprintf(g_pDefs, "type exid.1"); g_isCallExid = 1; }
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
			  type data			{ fprintf(g_pTarget, "data");	if (g_isCallExid == 1) fprintf(g_pDefs, "type"); }
			  callarg

type		: VOID | PTR | I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | F32 | F64
data		: NULLPTR | STR | INT | FLOAT | ID

%%

int main(int num_args, char **args) {
	setbuf(stdout, NULL);
	if (num_args < 2) {
		fprintf(stderr, "\e[91m[Fatal error]\e[0m no input files.\n");
		return 1;
	}
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
	for (int i = 1; i < num_args; ++i) {
		FILE *p_file = fopen(args[i], "r");
		if (p_file == NULL) {
			fprintf(stderr, "\e[91m[IO error]\e[0m %s not opened.\n", args[i]);
			return 1;
		}
		yyin = p_file;
		yyparse();
		fclose(p_file);
	}
	fclose(g_pDefs);
	fclose(g_pTarget);
	fclose(g_pHeader);
	return 0;
}
