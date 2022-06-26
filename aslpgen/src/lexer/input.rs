use super::*;

pub fn input() -> Result<(DFAutomata<usize>, Vec<String>), String> {
    // Read asl file
    let reader = BufReader::new(
        File::open(Path::new("./lexer.asl"))
            .map_err(|_| String::from("[fatal error] lexer.asl not found."))?,
    );
    let lines = reader.lines().map(|n| n.unwrap()).collect::<Vec<String>>();
    // Create NFAs
    let mut labels = Vec::new();
    let mut nfas = Vec::new();
    let mut cnt: usize = 0;
    for v in lines.into_iter() {
        if v.len() > 0 && v.chars().nth(0).unwrap() != '#' {
            let splited = v.split_whitespace().collect::<Vec<&str>>();
            if splited.len() < 2 {
                return Err(format!("[Syntax error] RE not found. : {} th token", cnt));
            }
            let restr = process_restr(splited[1])?;
            labels.push(String::from(splited[0]));
            nfas.push(regex::parse_nfa(regex::parse_regex(restr), cnt));
            cnt += 1;
        }
    }
    // Connect NFAs
    let mut itr = nfas.into_iter();
    let mut nfa = itr.next().unwrap();
    for i in itr {
        nfa = nfa.connect(&i);
    }
    // Return DFA
    Ok((DFAutomata::from(&nfa), labels))
}
fn process_restr(restr: &str) -> Result<String, String> {
    let mut res = String::new();
    let mut chars = restr.chars();
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let start = chars.next().unwrap() as u8;
                if chars.next().unwrap() != '-' {
                    return Err(String::from("[Syntax error] '-' expected."));
                }
                let end = chars.next().unwrap() as u8;
                if chars.next().unwrap() != ']' {
                    return Err(String::from("[Syntax error] ']' expected."));
                }
                res.push('(');
                for i in start..end {
                    res.push(i as char);
                    res.push('|');
                }
                res.push(end as char);
                res.push(')');
            }
            '$' => match chars.next().unwrap() {
                '_' => {
                    res.push('\\');
                    res.push('_');
                }
                '$' => res.push('$'),
                's' => res.push(' '),
                't' => res.push('\t'),
                'n' => res.push('\n'),
                n => return Err(format!("[Syntax error] invalid escape sequence. : {}", n)),
            },
            _ => res.push(c),
        }
    }
    Ok(res)
}
