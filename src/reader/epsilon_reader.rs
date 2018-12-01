use reader::*;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::epsilon_trace;
use trees::*;

pub struct EpsilonReader;

impl<Tk: Token> Reader<Tk> for EpsilonReader {
    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult { success: Some(epsilon_trace()), ongoing: None }
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

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}
