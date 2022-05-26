use super::*;

impl Program {
    pub fn format(&self) -> String {
        let f = format!("{:?}\n", self);
        let f = f.replace("Some(", "");
        let f = f.replace("[", "[ ");
        let f = f.replace("]", "] ");
        let f = f.replace(",", " ,");
        let f = f.replace("})", "}");
        let f = f.replace("}]", "} ]");
        let f = f.replace(")]", ") ]");
        let f = f.replace("}\n", "}");
        let words = f.split(' ').collect::<Vec<&str>>();
        let mut res = String::from("\n");
        let mut indent = 0;
        let mut no_indent = 0;
        for i in words {
            match i {
                "," if no_indent == 0 => {
                    res.push('\n');
                    res.push_str(&format_indent(indent));
                }
                "{" | "[" if no_indent == 0 => {
                    indent += 1;
                    res.push_str(i);
                    res.push('\n');
                    res.push_str(&format_indent(indent));
                }
                "}" | "]" if no_indent == 0 => {
                    indent -= 1;
                    res.push('\n');
                    res.push_str(&format_indent(indent));
                    res.push_str(i);
                }
                "CallArg" | "Type" | "Data" => {
                    no_indent += 1;
                    res.push_str(i);
                    res.push(' ');
                }
                "}" => {
                    no_indent -= 1;
                    res.push_str(i);
                    res.push(' ');
                }
                _ if i.ends_with(":") => (),
                _ => {
                    res.push_str(i);
                    res.push(' ');
                }
            }
        }
        res += "\n";
        res.replace(" \n", "\n")
    }
}
fn format_indent(indent: usize) -> String {
    let mut res = String::new();
    for _ in 0..indent {
        res.push(' ');
        res.push(' ');
    }
    res
}
