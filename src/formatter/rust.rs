use super::*;

impl Program {
    pub fn format_for_rust(&self) -> String {
        let mut res = String::new();
        self.blocks.format(&mut res);
        res
    }
}
impl Blocks {
    fn format(&self, res: &mut String) {
        for i in &self.blocks {
            i.format(res);
        }
    }
}
impl Block {
    fn format(&self, res: &mut String) {
        match self {
            Block::FunBlock { t, id, logics } => {
                let id_str = match id {
                    Token::Id(n) => n,
                    n => panic!("[Formatter error] {:?} is not id.", n),
                };
                res.push_str(&format!("fn {}() -> {} {{\n", id_str, t.get_format()));
                if let Some(n) = logics {
                    n.format(res);
                }
                res.push_str("}\n");
            }
        }
    }
}
impl Logics {
    fn format(&self, res: &mut String) {
        for i in &self.logics {
            i.format(res);
        }
    }
}
impl Logic {
    fn format(&self, res: &mut String) {
        match self {
            Logic::CallLogic { t: _, id, args } => {
                let id_str = match id {
                    Token::Id(n) => n,
                    n => panic!("[Formatter error] {:?} is not id.", n),
                };
                res.push_str(&format!("    {}(", id_str));
                if let Some(n) = args {
                    n.format(res);
                }
                res.push_str(");\n");
            }
        }
    }
}
impl CallArgs {
    fn format(&self, res: &mut String) {
        let mut is_needed_comma = false;
        for i in &self.args {
            if is_needed_comma {
                res.push_str(", ");
            } else {
                is_needed_comma = true;
            }
            i.format(res);
        }
    }
}
impl CallArg {
    fn format(&self, res: &mut String) {
        res.push_str(&self.d.get_format());
    }
}
impl Type {
    fn get_format(&self) -> String {
        use Token::*;
        String::from(match &self.value {
            Void => "()",
            Ptr => "usize",
            I32 => "i32",
            U32 => "u32",
            n => panic!("[Formatter error] {:?} is not type.", n),
        })
    }
}
impl Data {
    fn get_format(&self) -> String {
        use Token::*;
        match &self.value {
            Nullptr => String::from("0"),
            Str(n) | Int(n) | Float(n) | Id(n) => n.clone(),
            n => panic!("[Formatter error] {:?} is not data.", n),
        }
    }
}
