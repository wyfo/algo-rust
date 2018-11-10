use list::Stack;
use reader::*;
use std::rc::Rc;
use symbols::Tag;
use traces::Policy;
use traces::Trace;
use trees::*;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use itertools::Itertools;
use reader::epsilon_reader::EpsilonReader;

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
    fn new(reader: Rc<dyn Reader<Tk>>) -> Self {
        OptionalReader { reader, eps: EpsilonReader }
    }
}

impl<Tk: Token + 'static> Reader<Tk> for OptionalReader<Tk> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        EpsilonReader.epsilon(this)
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        read(&self.reader, token)
    }
}

impl<Tk: Token> TreeBuilder for OptionalReader<Tk> {
    fn tag(&self) -> Tag {
        None
    }

    fn leaf_builder(&self) -> LeafBuilder {
        unimplemented!()
    }

    fn switch_builder(&self, case: usize) -> SwitchBuilder {
        if case == 0 {
            SwitchBuilder::Case(&self.eps, None)
        } else {
            SwitchBuilder::Case(self.reader.as_tree_builder(), None)
        }
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}
