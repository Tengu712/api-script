use super::*;

pub struct LChar(pub char);
impl LChar {
    pub fn new_box(chr: char) -> Box<Self> {
        Box::new(Self(chr))
    }
}
impl Regex for LChar {
    fn assemble(&self, context: Context) -> (Context, NFAFrag) {
        let (context, start) = context.next();
        let (context, end) = context.next();
        let mut frag = NFAFrag::new(start);
        frag.accepts.insert(end);
        frag.connect(start, self.0, end);
        (context, frag)
    }
}
