use super::*;

pub struct Star(pub Box<dyn Regex>);
impl Star {
    pub fn new_box(op: Box<dyn Regex>) -> Box<Self> {
        Box::new(Self(op))
    }
}
impl Regex for Star {
    fn assemble(&self, context: Context) -> (Context, NFAFrag) {
        let (context, frag_trg) = self.0.assemble(context);
        let (context, start) = context.next();
        let mut frag = NFAFrag::new(start);
        frag.accepts = frag_trg.accepts.clone();
        frag.accepts.insert(start);
        frag.map = frag_trg.map.clone();
        frag.connect(start, '\0', frag_trg.start);
        for state in frag_trg.accepts.iter() {
            frag.connect(*state, '\0', frag_trg.start);
        }
        (context, frag)
    }
}
