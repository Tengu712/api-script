use super::*;

pub struct Concat(Regex, Regex);
impl Concat {
    pub fn new_box(op1: Regex, op2: Regex) -> Box<Self> {
        Box::new(Self(op1, op2))
    }
}
impl RegexImpl for Concat {
    fn assemble(&self, context: Context) -> (Context, NFAFrag) {
        let (context, frag1) = self.0.assemble(context);
        let (context, frag2) = self.1.assemble(context);
        let mut frag = NFAFrag::compose(&frag1, &frag2, frag1.start);
        frag.accepts = frag2.accepts;
        for state in frag1.accepts.iter() {
            frag.connect(*state, '\0', frag2.start);
        }
        (context, frag)
    }
}
impl core::fmt::Debug for Concat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Concat({:?}, {:?})", self.0, self.1)
    }
}
