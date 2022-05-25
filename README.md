# win32api-script

## 概要(Outline)

Win32APIはDLLで提供されている。ということは、DLLが扱える言語なら、Win32APIが扱える。

これを実証するために、Win32APIを記述するために特化したスクリプト言語と、様々な言語に対応したクロスコンパイラを作成する。

Win32API is served with DLL files. So we can use Win32API in any language that can link DLL.

The repository manages the cross compiler of the my script that's specialized to use Win32API.

## 文法

以下にBNF記法のような記法によって文法を示す。

Grammer written in BNF-like-format is shown below.

```
<Program> ::= indent <Block> indent $
<Block> ::= fun <Function> (indent <Block> | "")
<Type> ::= ptr | i32 | u32
<Data> ::= nullptr | str | int | float | id
<Function> ::= id (deferent_indent logic deferent_indent <Logic> | "")
<Logic> ::= call <Call> (same_indent <Logic> | "")
<Call> ::= id (deferent_indent <CallArgs> | "")
<CallArgs> ::= <Type> <Data> (same_indent <CallArgs> | "")
```