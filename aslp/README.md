# api-script lexer/parser

api-script compiler用のパッケージクレート。

api-script lexer/parser generatorによって生成されたlexer.rsとparser.rsをまとめたもの。

```
$ cargo new --lib aslp
$ mv aslpgen/lexer.rs aslp/src/lexer.rs
$ mv aslpgen/parser.rs aslp/src/parser.rs
$ vim aslp/lib.rs
pub mod lexer;
pub mod parser;
```
