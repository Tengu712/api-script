# api-script

## 概要

Win32APIやVulkan等の「API」は、ユーザに対し機械的な呼び出しを要求する。

この機械的な呼び出しを簡潔なスクリプトで記し、WindowsあるいはLinux向けの静的リンクライブラリに変換することを目的とする。

## asc

api-script compiler。

字句解析や構文解析のために、aslpクレートを用いる。

## aslp

api-script lexer/parser。

aslpgenによって生成されたオートマトンをパッケージクレートとしてまとめたもの。

## aslpgen

api-script lexer/parser generator。

正規表現をオートマトンに変換し、lexer.rsとparser.rsを出力する。
