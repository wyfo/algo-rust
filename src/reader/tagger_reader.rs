use reader::*;
use std::rc::Rc;
use symbols::Symbol;
use symbols::Tag;
use trees::*;

#[derive(Debug)]
pub struct TaggerReader<Tk: Token> {
    reader: Rc<dyn Reader<Tk>>,
    sym: Symbol,
}

impl<Tk: Token + 'static> Reader<Tk> for TaggerReader<Tk> {
    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        epsilon(&self.reader)
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, _: Tk) -> ReadingResult<Tk> {
        unimplemented!()
    }
}

impl<Tk: Token> TreeBuilder for TaggerReader<Tk> {
    fn tag(&self) -> Tag {
        Some(self.sym)
    }

    fn is_volatile(&self) -> VolatileBuilder {
        Some((self, Some(self.sym)))
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}

