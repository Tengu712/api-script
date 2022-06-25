# api-script compiler

## 概要

api-scriptスクリプト(.as)を静的リンクライブラリに変換するコンパイラ。

## サンプルコード

sampleディレクトリの各ファイルを参照。

## 文法

以下にBNF記法のような記法によって文法を示す。

```
<Program> ::= indent <Blocks> $
<Blocks> ::= <Block> (same_indent <Blocks> | "")
<Block> ::= fun <FunTree>
<FunTree> ::= 
  <Type> id (deferent_indent1
    (  args deferent_indent2 <Args>
          (deferent_indent1 logic deferent_indent3 <Logic> | "")
     | logic deferent_indent3 <Logic>)
<Logics> ::= <Logic> (same_indent <Logics> | "")
<Logic> ::= call <Call> | let <Let> | return <Data>
<Call> ::= id <Type> (deferent_indent <Args> | "")
<Let> ::= id (<Arg> | call <Call>)
<Args> ::= <Arg> (same_indent <Args> | "")
<Arg> ::= <Type> <Data> | struct <Args>
<Type> ::= void | ptr | i16 | i32 | u16 | u32 | f32
<Data> ::= nullptr | str | int | hex | float | id | refid
```
