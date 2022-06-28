%{
#include <stdio.h>
int g_curLine;
int g_curChar;
int yylex();
void yyerror(char *msg) {
    fprintf(stderr, "[Parsing error] %s : %d line, %d char\n", msg, g_curLine, g_curChar);
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
function	: FUN type ID INDENT logic DEDENT
logic		: LOGIC INDENT call DEDENT
call		: CALL type functionid
			| CALL type functionid INDENT callarg DEDENT
callarg		:
			| type data callarg

functionid	: ID | EXID
type		: VOID | PTR | I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | F32 | F64
data		: NULLPTR | STR | INT | FLOAT | ID

%%

int main(void) {
    yyparse();
    return 0;
}
