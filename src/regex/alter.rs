use super::*;

pub struct Alter(Regex, Regex);
impl Alter {
    pub fn new_box(op1: Regex, op2: Regex) -> Box<Self> {
        Box::new(Self(op1, op2))
    }
}
impl RegexImpl for Alter {
    fn assemble(&self, context: Context) -> (Context, NFAFrag) {
        let (context, frag1) = self.0.assemble(context);
        let (context, frag2) = self.1.assemble(context);
        let (context, start) = context.next();
        let mut frag = NFAFrag::compose(&frag1, &frag2, start);
        frag.len = context.0;
        frag.accepts = &frag1.accepts | &frag2.accepts;
        frag.connect(start, 0, frag1.start);
        frag.connect(start, 0, frag2.start);
        (context, frag)
    }
}
impl core::fmt::Debug for Alter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Alter({:?}, {:?})", self.0, self.1)
    }
}
