use super::*;

pub fn output(dfa: DFAutomata<usize>, labels: Vec<String>) -> Result<(), String> {
    let mut writer = BufWriter::new(
        File::create(Path::new("./lexer.rs"))
            .map_err(|_| String::from("[Fatal error] lexer.rs not created."))?,
    );
    write_lexer(
        &mut writer,
        String::from("use std::collections::HashMap;\n"),
    )?;
    // Token
    write_lexer(&mut writer, String::from("#[derive(Debug, Clone)]\n"))?;
    write_lexer(&mut writer, String::from("pub enum Token {\n"))?;
    for i in labels.iter() {
        write_lexer(&mut writer, format!("    {},\n", i.clone()))?;
    }
    write_lexer(&mut writer, String::from("}\n"))?;
    // Start
    write_lexer(&mut writer, format!("const START: usize = {};", dfa.start))?;
    // Accepts
    write_lexer(
        &mut writer,
        format!(
            "pub const ACCEPTS: [(usize, Token); {}] = [\n",
            dfa.accepts.len()
        ),
    )?;
    for (k, v) in dfa.accepts.into_iter() {
        let label_splited = labels[v].split('(').collect::<Vec<&str>>();
        let f = format!(
            "    ({}, Token::{}{}),\n",
            k,
            label_splited[0],
            if label_splited.len() < 2 {
                ""
            } else if label_splited[1] == "usize)" {
                "(0)"
            } else if label_splited[1] == "String)" {
                "(String::new())"
            } else {
                panic!("[Syntax error] invalid attribute : {}", label_splited[1]);
            }
        );
        write_lexer(&mut writer, f)?;
    }
    // Transition
    write_lexer(
        &mut writer,
        format!(
            "];\npub const TRANSITION: [[usize; 256]; {}] = [\n",
            dfa.transition.len()
        ),
    )?;
    for i in dfa.transition {
        write_lexer(&mut writer, format!("    {:?},\n", i))?;
    }
    write_lexer(&mut writer, String::from("];\n"))?;
    // RuntimeLexer
    write_lexer(
        &mut writer,
        String::from(
            "\
pub struct RuntimeLexer {
    accepts: HashMap<usize, Token>,
    transition: Vec<&'static [usize; 256]>,
}
impl RuntimeLexer {
    pub fn new() -> Self {
        let accepts = HashMap::from(ACCEPTS);
        let mut transition = Vec::with_capacity(TRANSITION.len());
        for i in &TRANSITION {
            transition.push(i);
        }
        Self {
            accepts,
            transition,
        }
    }
    pub fn start_with<I>(&self, trg: &mut std::iter::Peekable<I>) -> Option<(Token, String)>
    where
        I: Iterator<Item = u8>,
    {
        let mut buf = String::new();
        let mut flag = false;
        let mut state = START;
        while let Some(c) = trg.peek() {
            let tmp = self.transition[state][*c as usize];
            if tmp == 0 {
                break;
            } else {
                buf.push(*c as char);
                flag = true;
                state = tmp;
                let _ = trg.next();
            }
        }
        if flag {
            self.accepts.get(&state).map(|n| (n.clone(), buf))
        } else {
            None
        }
    }
}            
",
        ),
    )?;
    Ok(())
}
fn write_lexer(writer: &mut BufWriter<File>, f: String) -> Result<(), String> {
    writer
        .write_all(f.as_bytes())
        .map_err(|_| String::from("[IO error] something is wrong with output lexer.rs."))
}
