# System
Space           $s$s*
Comment         #_*$n
Newline         $n
Indent(usize)   :[0-9]:

# Symbol
Fun             fun
Args            args
Logic           logic
Call            call

# Type
Void            void
Ptr             ptr
I8              i8
I16             i16
I32             i32
I64             i64
U8              u8
U16             u16
U32             u32
U64             u64

# Data
Nullptr         nullptr
Str(String)     "_"
Int(String)     [0-9][0-9]*
Float(String)   [0-9][0-9]*.[0-9][0-9]*
Id(String)      ([a-z]|[A-Z][a-z]|[A-Z]|$_)([0-9]|[a-z]|[A-Z][a-z]|[A-Z]|$_)*
