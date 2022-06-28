%{
#include <stdio.h>
#include "asc.tab.h"
int g_curIndent = 0;
int yywrap(void) {
	return 1;
}
void check_indent_error(int len) {
	if (len % 4 != 0) {
		fprintf(stderr, "[Syntax error] num of indent spaces must be multiples of 4.\n");
		exit(1);
	}
	if (len > g_curIndent && len != g_curIndent + 4) {
		fprintf(
			stderr,
			"[Syntax error] an indent must be 4 spaces. : %d -> %d\n",
			g_curIndent,
			len
		);
		exit(1);
	}
}
%}

indent			" "*
newline			\n|\r|\r\n
comment			#.*{newline}
int				0|[1-9][0-9]*
id				[a-zA-Z_][0-9a-zA-Z_]+

%x code

%%

<INITIAL>{indent}{comment}	;
<INITIAL>{indent}{newline}	;
<INITIAL>{indent}			{
								check_indent_error(yyleng);
								if (yyleng > g_curIndent) {
									g_curIndent = yyleng;
									printf("\nINDENT ");
									BEGIN code;
									return INDENT;
								} else if (yyleng < g_curIndent) {
									g_curIndent -= 4;
									for (int i = 0; i < g_curIndent - yyleng; ++i) {
										unput(' ');
									}
									printf("\nDEDENT ");
									BEGIN code;
									return DEDENT;
								} else {
									BEGIN code;
								}
							}
<INITIAL>.					{
								unput(yytext[0]);
								if (g_curIndent == 0) {
									BEGIN code;
								} else {
									printf("\nDEDENT ");
									g_curIndent -= 4;
									return DEDENT;
								}
							}
<code>" "+					;
<code>{comment}				{ BEGIN INITIAL; }
<code>{newline}				{ BEGIN INITIAL; }

<code>fun					{ printf("FUN "); return FUN; }
<code>args					{ printf("ARGS "); return ARGS; }
<code>logic					{ printf("LOGIC "); return LOGIC; }
<code>call					{ printf("CALL "); return CALL; }

<code>void					{ printf("VOID "); return VOID; }
<code>ptr					{ printf("PTR "); return PTR; }
<code>i8					{ printf("I8 "); return I8; }
<code>i16					{ printf("I16 "); return I16; }
<code>i32					{ printf("I32 "); return I32; }
<code>i64					{ printf("I64 "); return I64; }
<code>u8					{ printf("U8 "); return U8; }
<code>u16					{ printf("U16 "); return U16; }
<code>u32					{ printf("U32 "); return U32; }
<code>u64					{ printf("U64 "); return U64; }
<code>f32					{ printf("F32 "); return F32; }
<code>f64					{ printf("F64 "); return F64; }

<code>nullptr				{ printf("NULLPTR "); return NULLPTR; }
<code>\".*\"				{ printf("STR "); return STR; }
<code>{int}					{ printf("INT "); return INT; }
<code>{int}\.{int}			{ printf("FLOAT "); return FLOAT; }
<code>{id}					{ printf("ID "); return ID; }
<code>{id}\.{id}			{ printf("EXID "); return EXID; }

<code>.						{ fprintf(stderr, "[Syntax error] invalid token. : %s\n", yytext); exit(1); }
<<EOF>>						{
								if (g_curIndent == 0) {
									printf("\nEOF\n");
									return EOFILE;
								} else {
									printf("\nDEDENT ");
									g_curIndent -= 4;
									return DEDENT;
								}
							}

%%
