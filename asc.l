%{
#include <stdio.h>
#include <string.h>
#include "asc.tab.h"
int g_curIndent = 0;
int g_curLine = 1;
int g_curChar = 1;
int g_indentWidth = 2;
void add_cur_chara() {
	g_curChar += yyleng;
}
int yywrap(void) {
	return 1;
}
int check_indent_error() {
	if (yyleng % g_indentWidth != 0) {
		fprintf(
			stderr,
			"\e[91m[Syntax error]\e[0m num of indent spaces must be multiples of %d. : %d line\n",
			g_indentWidth,
			g_curLine
		);
		return 0;
	}
	if (yyleng > g_curIndent && yyleng != g_curIndent + g_indentWidth) {
		fprintf(
			stderr,
			"\e[91m[Syntax error]\e[0m an indent must be %d spaces. : %d -> %d : %d line\n",
			g_indentWidth,
			g_curIndent,
			yyleng,
			g_curChar
		);
		return 0;
	}
	return 1;
}
%}

indent			" "*
newline			\n|\r|\r\n
comment			#.*{newline}
int				0|[1-9][0-9]*
id				[a-zA-Z][0-9a-zA-Z_]*

%x code inline_s

%%

<INITIAL>{indent}{comment}	|
<INITIAL>{indent}{newline}	{ g_curChar = 0; ++g_curLine; }
<INITIAL>{indent}			{
								if (check_indent_error() == 0)
								    yyterminate();
								add_cur_chara();
								if (yyleng > g_curIndent) {
									g_curIndent = yyleng;
									BEGIN code;
									return INDENT;
								} else if (yyleng < g_curIndent) {
									g_curIndent -= g_indentWidth;
									for (int i = 0; i < g_curIndent - yyleng; ++i) {
										unput(' ');
									}
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
									g_curIndent -= g_indentWidth;
									return DEDENT;
								}
							}
<code>" "+					{ add_cur_chara(); }
<code>{comment}				|
<code>{newline}				{ g_curChar = 0; ++g_curLine; BEGIN INITIAL; }
<code>"("					{ add_cur_chara(); BEGIN inline_s; }
<inline_s>.*")"				{
								add_cur_chara();
								yytext[yyleng - 1] = '\0';
								yylval.data = yytext;
								BEGIN code;
								return INLINE;
							}

<code>fun					{ add_cur_chara(); return FUN; }
<code>args					{ add_cur_chara(); return ARGS; }
<code>logic					{ add_cur_chara(); return LOGIC; }
<code>return				{ add_cur_chara(); return RETURN; }
<code>call					{ add_cur_chara(); return CALL; }
<code>if					{ add_cur_chara(); return IF; }
<code>elif					{ add_cur_chara(); return ELIF; }
<code>else					{ add_cur_chara(); return ELSE; }

<code>void					{ add_cur_chara(); return VOID; }
<code>ptr					{ add_cur_chara(); return PTR; }
<code>i8					{ add_cur_chara(); return I8; }
<code>i16					{ add_cur_chara(); return I16; }
<code>i32					{ add_cur_chara(); return I32; }
<code>i64					{ add_cur_chara(); return I64; }
<code>u8					{ add_cur_chara(); return U8; }
<code>u16					{ add_cur_chara(); return U16; }
<code>u32					{ add_cur_chara(); return U32; }
<code>u64					{ add_cur_chara(); return U64; }
<code>f32					{ add_cur_chara(); return F32; }
<code>f64					{ add_cur_chara(); return F64; }

<code>nullptr				{ add_cur_chara(); return NULLPTR; }
<code>\".*\"				{ add_cur_chara(); yylval.data = strdup(yytext); return STR; }
<code>{int}					{ add_cur_chara(); yylval.data = strdup(yytext); return INT; }
<code>{int}\.{int}			{ add_cur_chara(); yylval.data = strdup(yytext); return FLOAT; }
<code>{id}					{ add_cur_chara(); yylval.data = strdup(yytext); return ID; }
<code>{id}\.{id}			{
								add_cur_chara();
								int i;
								for (i = 0; i < strlen(yytext); ++i) {
									if (yytext[i] == '.')
										break;
								}
								char *s_libname = (char*)malloc(sizeof(char) * i);
								char *s_id = (char*)malloc(sizeof(char) * (strlen(yytext) - i - 1));
								strncpy(s_libname, yytext, i);
								strncpy(s_id, yytext + i + 1, (strlen(yytext) - i - 1));
								ExID tmp = { s_libname, s_id };
								yylval.exid = tmp;
								return EXID;
							}

<code>.						{
								fprintf(
									stderr,
									"\e[91m[Syntax error]\e[0m invalid token. : %s : %d line, %d char\n",
									yytext,
									g_curLine,
									g_curChar
								);
								yyterminate();
							}
<<EOF>>						{
								if (g_curIndent == 0)
									return EOFILE;
								else {
									g_curIndent -= g_indentWidth;
									return DEDENT;
								}
							}

%%
