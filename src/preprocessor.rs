pub fn preprocess(lines: Vec<String>) -> Result<Vec<u8>, String> {
    let mut res = Vec::new();
    for line in lines {
        let len_prev = res.len();
        let mut flag = 0;
        for i in line.as_bytes() {
            if flag == 0 && *i as char == ':' {
                flag = -1;
            }
            if flag >= 0 && *i as char != ' ' && *i as char != '#' {
                res.push(':' as u8);
                res.extend(flag.to_string().as_bytes());
                res.push(':' as u8);
                res.push(' ' as u8);
                flag = -1;
            }
            match *i as char {
                '#' => break,
                ' ' if flag >= 0 => {
                    flag += 1;
                }
                _ => res.push(*i),
            }
        }
        if len_prev != res.len() {
            res.push('\n' as u8);
        }
    }
    Ok(res)
}

#[test]
fn test1() {
    let code = "\
# Hello World with MessageBoxA
fun void hello_world
    logic
        call i32 user32.MessageBoxA
            ptr nullptr
            ptr \"Hello World!\"
            ptr \"title\"
            u32 0
";
    let expect = "\
:0: fun void hello_world
:4: logic
:8: call i32 user32.MessageBoxA
:12: ptr nullptr
:12: ptr \"Hello World!\"
:12: ptr \"title\"
:12: u32 0
";
    let lines = code.split('\n').map(|n| String::from(n)).collect();
    let src_str = preprocess(lines).unwrap();
    assert_eq!(src_str, expect.as_bytes());
}
#[test]
fn test2() {
    let code = "\
# Hello World with MessageBoxA
fun void hello_world
    logic #LOGIC#
:7: call i32 user32.MessageBoxA
            ptr nullptr
:12: ptr \"Hello World!\"
            ptr \"title\"
            u32 0
";
    let expect = "\
:0: fun void hello_world
:4: logic 
:7: call i32 user32.MessageBoxA
:12: ptr nullptr
:12: ptr \"Hello World!\"
:12: ptr \"title\"
:12: u32 0
";
    let lines = code.split('\n').map(|n| String::from(n)).collect();
    let src_str = preprocess(lines).unwrap();
    assert_eq!(src_str, expect.as_bytes());
}
