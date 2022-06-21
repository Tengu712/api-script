use super::*;

pub struct Alter(pub Box<dyn Regex>, pub Box<dyn Regex>);
impl Alter {
    pub fn new_box(op1: Box<dyn Regex>, op2: Box<dyn Regex>) -> Box<Self> {
        Box::new(Self(op1, op2))
    }
}
impl Regex for Alter {
    fn assemble(&self, context: Context) -> (Context, NFAFrag) {
        let (context, frag1) = self.0.assemble(context);
        let (context, frag2) = self.1.assemble(context);
        let (context, start) = context.next();
        let mut frag = NFAFrag::compose(&frag1, &frag2, start);
        frag.accepts = &frag1.accepts | &frag2.accepts;
        frag.connect(start, '\0', frag1.start);
        frag.connect(start, '\0', frag2.start);
        (context, frag)
    }
}
