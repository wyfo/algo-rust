use reader::*;
use std::rc::Rc;
use symbols::Tag;
use trees::*;
use traces::*;
use list::*;
use std::any::Any;
use std::ops::Index;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ConditionalTokenReader<Tk: Token, M: Index<usize, Output=ReadingResult<Tk>> + Debug> {
    pub tag: Tag,
    pub matching: M,
}

impl<Tk: Token + 'static, M: Index<usize, Output=ReadingResult<Tk>> + Debug + 'static> Reader<Tk> for ConditionalTokenReader<Tk, M> {
    fn tag(&self) -> Tag {
        self.tag
    }

    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult { success: None, ongoing: Some(this.clone()) }
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.matching[token.id()].clone()
    }
}

impl<Tk: Token, M: Index<usize, Output=ReadingResult<Tk>> + Debug> TreeBuilder for ConditionalTokenReader<Tk, M> {
    fn leaf_builder(&self) -> LeafBuilder {
        LeafBuilder::Token(self.tag)
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}

impl<Tk: Token + 'static, M: Index<usize, Output=ReadingResult<Tk>> + Debug + 'static> AsAny for ConditionalTokenReader<Tk, M> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}