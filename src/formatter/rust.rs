use super::*;

impl Program {
    pub fn format_for_rust(&self) -> String {
        let mut body = String::new();
        let mut link = String::new();
        self.blocks.format(&mut body, &mut link);
        link + &body
    }
}
impl Blocks {
    fn format(&self, body: &mut String, link: &mut String) {
        for i in &self.blocks {
            i.format(body, link);
        }
    }
}
impl Block {
    fn format(&self, body: &mut String, link: &mut String) {
        match self {
            Block::FunBlock {
                t,
                id,
                args,
                logics,
            } => {
                let id_str = match id {
                    Token::Id(n) => n,
                    n => panic!("[Formatter error] {:?} is not id.", n),
                };
                let args_str = match args {
                    Some(n) => n.get_format(ArgFormat::DefArg),
                    None => String::new(),
                };
                body.push_str(&format!(
                    "fn {}({}) -> {} {{\n",
                    id_str,
                    args_str,
                    t.get_format()
                ));
                if let Some(n) = logics {
                    n.format(body, link);
                }
                body.push_str("}\n");
            }
        }
    }
}
impl Logics {
    fn format(&self, body: &mut String, link: &mut String) {
        for i in &self.logics {
            i.format(body, link);
        }
    }
}
impl Logic {
    fn format(&self, body: &mut String, link: &mut String) {
        match self {
            Logic::CallLogic { t, id, args } => self.format_calllogic(body, link, t, id, args),
        }
    }
    fn format_calllogic(
        &self,
        body: &mut String,
        link: &mut String,
        t: &Type,
        id: &Token,
        args: &Option<Args>,
    ) {
        let callargs_str = if let Some(n) = args {
            n.get_format(ArgFormat::CallArg)
        } else {
            String::from("")
        };
        let id_str = match id {
            Token::Id(n) => n,
            n => panic!("[Formatter error] {:?} is not id.", n),
        };
        let splited = id_str.split('.').collect::<Vec<&str>>();
        if splited.len() == 2 {
            let linkargs_str = if let Some(n) = args {
                n.get_format(ArgFormat::LinkArg)
            } else {
                String::from("")
            };
            link.push_str(&format!(
                "\
#[link(name=\"{}\")]
extern \"stdcall\" {{
    fn {}({}) -> {};
}}
",
                splited[0],
                splited[1],
                linkargs_str,
                t.get_format()
            ));
            body.push_str(&format!(
                "    unsafe {{ {}({}) }};\n",
                splited[1], callargs_str
            ))
        } else {
            body.push_str(&format!("    {}({});\n", id_str, callargs_str));
        }
    }
}
impl Args {
    fn get_format(&self, kind: ArgFormat) -> String {
        let mut res = String::new();
        let mut is_needed_comma = false;
        for i in &self.args {
            if is_needed_comma {
                res.push_str(", ");
            } else {
                is_needed_comma = true;
            }
            res.push_str(&format!("{}", i.get_format(kind.clone())));
        }
        res
    }
}
#[derive(Clone)]
enum ArgFormat {
    DefArg,
    LinkArg,
    CallArg,
}
impl Arg {
    fn get_format(&self, kind: ArgFormat) -> String {
        match kind {
            ArgFormat::DefArg => self.d.get_format() + ": " + &self.t.get_format(),
            ArgFormat::LinkArg => String::from("_: ") + &self.t.get_format(),
            ArgFormat::CallArg => self.d.get_format(),
        }
    }
}
impl Type {
    fn get_format(&self) -> String {
        use Token::*;
        String::from(match &self.value {
            Void => "()",
            Ptr => "i32",
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
            Str(n) => n.clone() + ".as_ptr() as i32",
            Int(n) | Float(n) | Id(n) => n.clone(),
            n => panic!("[Formatter error] {:?} is not data.", n),
        }
    }
}
