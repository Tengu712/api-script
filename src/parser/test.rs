use super::*;

impl Cell {
    fn format_(&self, res: &mut String, indent: usize, is_car: bool) {
        match self {
            Cell::Nil => (),
            Cell::Atom(n) => {
                res.push_str(&format!("{:?} ", n));
            }
            Cell::Pair(car, cdr) => {
                if is_car {
                    res.push('\n');
                    res.push_str(&format_indent(indent));
                }
                car.format_(res, indent + 1, true);
                cdr.format_(res, indent, false);
            }
        }
    }
    pub fn format(&self) -> String {
        let mut res = String::new();
        self.format_(&mut res, 0, true);
        res.push('\n');
        res
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
