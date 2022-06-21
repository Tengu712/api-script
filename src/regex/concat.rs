use super::*;

pub struct Concat(pub Box<dyn Regex>, pub Box<dyn Regex>);
impl Concat {
    pub fn new_box(op1: Box<dyn Regex>, op2: Box<dyn Regex>) -> Box<Self> {
        Box::new(Self(op1, op2))
    }
}
impl Regex for Concat {
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
