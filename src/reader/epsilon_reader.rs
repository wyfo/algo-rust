use reader::*;
use std::rc::Rc;
use trees::*;
use traces::new_traces;
use symbols::Tag;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

pub struct EpsilonReader;

impl<Tk: Token> Reader<Tk> for EpsilonReader {
    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult {success: Some(new_traces()), ongoing: None }
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, _: Tk) -> ReadingResult<Tk> {
        unimplemented!()
    }
}

impl Debug for EpsilonReader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "()")
    }
}

impl TreeBuilder for EpsilonReader {
    fn tag(&self) -> Tag {
        None
    }

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
