use super::*;

impl Tokens {
    pub fn format(&self) -> String {
        let mut res = String::new();
        for i in &self.tokens {
            match i {
                Token::Indent(_) => res = format!("{}\n{:?}", res, i),
                _ => res = format!("{} {:?}", res, i),
            }
        }
        res + "\n"
    }
}
