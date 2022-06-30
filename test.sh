#!/bin/sh
test () {
    ./asc -c sample/$1/program.as
    if [ "`cat a.c`" = "`cat sample/$1/a.c`" ] \
        && [ "`cat a.h`" = "`cat sample/$1/a.h`" ] \
        && [ "`cat a_.h`" = "`cat sample/$1/a_.h`" ]; \
    then
        echo "$1 is ok.";
    else
        echo "$1 is NOT ok."
    fi
}
test 01_empty_program
test 02_let_return
test 03_if
rm a_.h
rm a.c
rm a.h
