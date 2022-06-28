build: asc.y asc.l
	@bison -d asc.y
	@flex asc.l
	@gcc asc.tab.c lex.yy.c
