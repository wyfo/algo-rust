use list::Stack;
use reader::*;
use reader::epsilon_reader::EpsilonReader;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::epsilon_trace;
use traces::new_traces;
use traces::Policy;
use traces::Trace;
use trees::*;

pub struct OptionalReader<Tk: Token> {
    pub reader: Rc<dyn Reader<Tk>>,
    eps: EpsilonReader,
}

impl<Tk: Token> Debug for OptionalReader<Tk> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}?", self.reader)
    }
}

impl<Tk: Token> OptionalReader<Tk> {
    pub fn new(reader: Rc<dyn Reader<Tk>>) -> Self {
        OptionalReader { reader, eps: EpsilonReader }
    }
}

impl<Tk: Token + 'static> Reader<Tk> for OptionalReader<Tk> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult {
            success: Some(epsilon_trace()),
            ongoing: epsilon(&self.reader).ongoing,
        }
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        unimplemented!()
    }
}

impl<Tk: Token> TreeBuilder for OptionalReader<Tk> {
    fn tag(&self) -> Tag {
        None
    }

    fn is_volatile(&self) -> VolatileBuilder {
        Some((self.reader.as_tree_builder(), None))
    }

    fn switch_builder(&self, case: usize) -> SwitchBuilder {
        self.reader.switch_builder(case)
    }

    fn node_builder(&self) -> NodeBuilder {
        self.reader.node_builder()
    }
}
