%{
#include <stdio.h>
#include <stdlib.h>
int g_curLine;
int g_curChar;
FILE *yyin;
int yylex();
void yyerror(char *msg) {
	fprintf(
		stderr,
		"\n\e[91m[Parsing error]\e[0m %s : %d line, %d char\n",
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

program		: blocks EOFILE
					{ printf("end\n"); return 0; }
blocks		:
			| block blocks
block		: function

function	: FUN type ID funbody
funbody		: INDENT args logic DEDENT
			| INDENT logic DEDENT
args		: ARGS INDENT funarg DEDENT
funarg		:
			| type ID funarg
logic		: LOGIC INDENT call DEDENT
call		: CALL type functionid
			| CALL type functionid INDENT callarg DEDENT
callarg		:
			| type data callarg

functionid	: ID | EXID
type		: VOID | PTR | I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | F32 | F64
data		: NULLPTR | STR | INT | FLOAT | ID

%%

int main(int num_args, char **args) {
	setbuf(stdout, NULL);
	if (num_args < 2) {
		fprintf(stderr, "\e[91m[Fatal error]\e[0m no input files.\n");
		return 1;
	}
	for (int i = 1; i < num_args; ++i) {
		FILE *p_file = fopen(args[i], "r");
		if (p_file == NULL) {
			fprintf(stderr, "\e[91m[IO error]\e[0m %s not opened.\n", args[i]);
			return 1;
		}
		yyin = p_file;
		yyparse();
	}
	return 0;
}
