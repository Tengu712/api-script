pub fn preprocess(lines: Vec<String>) -> Vec<u8> {
    let mut res = Vec::new();
    for line in lines {
        process_line(line, &mut res);
    }
    res
}
fn process_line(line: String, res: &mut Vec<u8>) {
    // Skip white line
    if line.len() == 0 {
        return;
    }
    let mut itr = line.as_bytes().iter();
    // Detect indent
    if line.as_bytes()[0] != ':' as u8 {
        let mut cnt = 0;
        while let Some(i) = itr.next() {
            if *i == '#' as u8 {
                return;
            }
            if *i != ' ' as u8 {
                res.push(':' as u8);
                res.extend(cnt.to_string().as_bytes());
                res.push(':' as u8);
                res.push(' ' as u8);
                res.push(*i);
                break;
            }
            cnt += 1;
        }
    }
    // Push all
    for i in itr {
        res.push(*i);
    }
    res.push('\n' as u8);
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
    let src_str = preprocess(lines);
    assert_eq!(String::from_utf8(src_str).unwrap(), String::from(expect));
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
:4: logic #LOGIC#
:7: call i32 user32.MessageBoxA
:12: ptr nullptr
:12: ptr \"Hello World!\"
:12: ptr \"title\"
:12: u32 0
";
    let lines = code.split('\n').map(|n| String::from(n)).collect();
    let src_str = preprocess(lines);
    assert_eq!(String::from_utf8(src_str).unwrap(), String::from(expect));
}
