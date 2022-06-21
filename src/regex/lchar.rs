use super::*;

pub struct LChar(char);
impl LChar {
    pub fn new_box(chr: char) -> Box<Self> {
        Box::new(Self(chr))
    }
}
impl RegexImpl for LChar {
    fn assemble(&self, context: Context) -> (Context, NFAFrag) {
        let (context, start) = context.next();
        let (context, end) = context.next();
        let mut frag = NFAFrag::new(start);
        frag.accepts.insert(end);
        frag.connect(start, self.0, end);
        (context, frag)
    }
}
impl core::fmt::Debug for LChar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LChar({})", self.0)
    }
}
