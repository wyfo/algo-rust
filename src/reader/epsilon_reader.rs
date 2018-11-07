use reader::*;
use std::rc::Rc;
use trees::*;
use traces::new_traces;
use std::any::Any;

#[derive(Debug)]
pub struct EpsilonReader;

impl<Tk: Token> Reader<Tk> for EpsilonReader {
    fn tag(&self) -> Tag {
        None
    }

    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult {success: Some(new_traces()), ongoing: None }
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, _: Tk) -> ReadingResult<Tk> {
        unimplemented!()
    }
}

impl TreeBuilder for EpsilonReader {
    fn leaf_builder(&self) -> LeafBuilder {
        LeafBuilder::Epsilon(None)
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}
