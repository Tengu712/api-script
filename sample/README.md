# sample

## 01_empty_program.as

```c
#include "a_.h"
int main() {}
```

## 02_let_return.as

```c
#include "a_.h"
int hoge() {
    int res = 2;
    return res;
}
int main() {
    return hoge();
}

```
