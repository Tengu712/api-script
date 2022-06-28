%{
#include <stdio.h>
#include "asc.tab.h"
int g_curIndent = 0;
int g_curLine = 1;
int g_curChar = 1;
void add_cur_chara() {
	g_curChar += yyleng;
}
int yywrap(void) {
	return 1;
}
void check_indent_error() {
	if (yyleng % 4 != 0) {
		fprintf(
			stderr,
			"\n\e[91m[Syntax error]\e[0m num of indent spaces must be multiples of 4. : %d line\n",
			g_curLine
		);
		exit(1);
	}
	if (yyleng > g_curIndent && yyleng != g_curIndent + 4) {
		fprintf(
			stderr,
			"\n\e[91m[Syntax error]\e[0m an indent must be 4 spaces. : %d -> %d : %d line\n",
			g_curIndent,
			yyleng,
			g_curChar
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

<INITIAL>{indent}{comment}	|
<INITIAL>{indent}{newline}	{ g_curChar = 0; ++g_curLine; }
<INITIAL>{indent}			{
								check_indent_error();
								add_cur_chara();
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
<code>" "+					{ add_cur_chara(); }
<code>{comment}				|
<code>{newline}				{ g_curChar = 0; ++g_curLine; BEGIN INITIAL; }

<code>fun					{ printf("FUN "); add_cur_chara(); return FUN; }
<code>args					{ printf("ARGS "); add_cur_chara(); return ARGS; }
<code>logic					{ printf("LOGIC "); add_cur_chara(); return LOGIC; }
<code>call					{ printf("CALL "); add_cur_chara(); return CALL; }

<code>void					{ printf("VOID "); add_cur_chara(); return VOID; }
<code>ptr					{ printf("PTR "); add_cur_chara(); return PTR; }
<code>i8					{ printf("I8 "); add_cur_chara(); return I8; }
<code>i16					{ printf("I16 "); add_cur_chara(); return I16; }
<code>i32					{ printf("I32 "); add_cur_chara(); return I32; }
<code>i64					{ printf("I64 "); add_cur_chara(); return I64; }
<code>u8					{ printf("U8 "); add_cur_chara(); return U8; }
<code>u16					{ printf("U16 "); add_cur_chara(); return U16; }
<code>u32					{ printf("U32 "); add_cur_chara(); return U32; }
<code>u64					{ printf("U64 "); add_cur_chara(); return U64; }
<code>f32					{ printf("F32 "); add_cur_chara(); return F32; }
<code>f64					{ printf("F64 "); add_cur_chara(); return F64; }

<code>nullptr				{ printf("NULLPTR "); add_cur_chara(); return NULLPTR; }
<code>\".*\"				{ printf("STR "); add_cur_chara(); return STR; }
<code>{int}					{ printf("INT "); add_cur_chara(); return INT; }
<code>{int}\.{int}			{ printf("FLOAT "); add_cur_chara(); return FLOAT; }
<code>{id}					{ printf("ID "); add_cur_chara(); return ID; }
<code>{id}\.{id}			{ printf("EXID "); add_cur_chara(); return EXID; }

<code>.						{
								fprintf(
									stderr,
									"\n\e[91m[Syntax error]\e[0m invalid token. : %s : %d line, %d char\n",
									yytext,
									g_curLine,
									g_curChar
								);
								exit(1);
							}
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
