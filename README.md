# win32api-script

## 概要(Outline)

Win32APIはDLLで提供されている。ということは、DLLが扱える言語なら、Win32APIが扱える。

これを実証するために、Win32APIを記述するために特化したスクリプト言語と、様々な言語に対応したクロスコンパイラを作成する。

Win32API is served with DLL files. So we can use Win32API in any language that can link DLL.

The repository manages the cross compiler of the abstract script that's specialized to use Win32API.
