# win32api-script

## 概要(Outline)

Win32APIはDLLで提供されている。ということは、DLLが扱える言語なら、Win32APIが扱える。

これを実証するために、Win32APIを記述するために特化したスクリプト言語と、様々な言語に対応したクロスコンパイラを作成する。

## 使用方法

コマンドライン引数は以下のような書式を持つ。

```
file-path target-language ( output-file-path | "print" )
```

1. file-path : 翻訳対象のパス
2. target-language : 翻訳先の言語名
3. * output-file-path : 翻訳後のパス
   * print : 翻訳の結果を標準出力する

コンパイラ名がwascであるとすると、例えば01.wasを01.rsに翻訳する場合は、以下のコマンドを実行する。

```
> wasc 01.was rust 01.rs
```

## サンプルコード

sampleディレクトリの各ファイルを参照。

本ディレクトリでサンプルコードを実行するには、例えば01.wasをrustに翻訳し標準出力として結果を受け取るならば、以下のコマンドを実行する。

```
> cargo run sample/01.was rust print
```

## 文法

以下にBNF記法のような記法によって文法を示す。

```
<Program> ::= indent <Blocks> $
<Blocks> ::= <Block> (same_indent <Blocks> | "")
<Block> ::= fun <Type> id (deferent_indent logic deferent_indent <Logic> | "")
<Logics> ::= <Logic> (same_indent <Logics> | "")
<Logic> ::= call <Type> id (deferent_indent <CallArgs> | "")
<CallArgs> ::= <CallArg> (same_indent <CallArgs> | "")
<CallArg> ::= <Type> <Data>
<Type> ::= void | ptr | i32 | u32
<Data> ::= nullptr | str | int | float | id
```
