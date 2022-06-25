# api-script lexer/parser generator

## 概要

字句解析・構文解析用のオートマトンを生成するプログラム。

lexer.aslからlexer.rsを、parser.aslからparser.rsを生成する。

## lexer.asl

`#`で始まる行をコメントとみなす。また、空白行もスキップされる。

各行は`<Tokenname>[(type)]<spaces or tabs><regular-expression>`で構成される。ただし、正規表現において以下のエスケープシーケンスを用いる。

| エスケープシーケンス | 意味 |
| ----- | ----- |
| $s | スペース |
| $t | タブ |
| $n | \\n |
| $_ | \\_ |
| $$ | $ |

以下の三つが生成される。

* トークン列挙型 Token
* 受理状態の配列 ACCEPTS: [(usize, Token); _]
* 遷移関数(配列) TRANSITION: [[usize; 256]; _]
