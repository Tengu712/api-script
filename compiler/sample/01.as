# Hello World with MessageBoxA
fun void hello_world
    logic
        call i32 user32.MessageBoxA
            ptr nullptr
            ptr "Hello World!"
            ptr "title"
            u32 0
