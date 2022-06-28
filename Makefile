build: asc.y asc.l
	@bison -d asc.y
	@flex asc.l
	@gcc -o asc asc.tab.c lex.yy.c
