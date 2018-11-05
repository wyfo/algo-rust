use reader::*;
use std::rc::Rc;
use trees::*;
use symbols::Tag;
use std::any::Any;

#[derive(Debug)]
pub struct RefReader<Tk: Token> {
    val: Option<Rc<dyn Reader<Tk>>>
}

impl<Tk: Token + 'static> Reader<Tk> for RefReader<Tk> {
    fn tag(&self) -> Tag {
        self.val.as_ref().unwrap().tag()
    }

    fn epsilon(&self, _: &Rc<Reader<Tk>>) -> ReadingResult<Tk> {
        epsilon(self.val.as_ref().unwrap())
    }

    fn read(&self, _: &Rc<Reader<Tk>>, _: Tk) -> ReadingResult<Tk> {
        unimplemented!()
    }
}

impl<Tk: Token> TreeBuilder for RefReader<Tk> {
    fn is_volatile(&self) -> VolatileBuilder {
        Some((self, None))
    }

    fn leaf_builder(&self) -> LeafBuilder {
        unimplemented!()
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}

impl<Tk: Token + 'static> AsAny for RefReader<Tk> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}