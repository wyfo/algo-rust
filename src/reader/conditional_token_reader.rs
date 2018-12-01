use reader::*;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::*;
use trees::*;

pub struct ConditionalTokenReader<Tk: Token> {
    pub matching: Vec<ReadingResult<Tk>>,
    pub tag: Tag,
}

impl<Tk: Token> Debug for ConditionalTokenReader<Tk> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "_")
    }
}

impl<T: Token> ConditionalTokenReader<T> {
    pub fn success<Tk: Token>() -> ReadingResult<Tk> {
        ReadingResult { success: Some(token_trace()), ongoing: None }
    }
    pub fn fail<Tk: Token>() -> ReadingResult<Tk> {
        ReadingResult::none()
    }
    pub fn include<Tk: Token>(tokens: Vec<Tk>, nb: usize, tag: Tag) -> Self {
        let mut matching = vec![Self::fail(); nb];
        for tk in tokens { matching[tk.id()] = Self::success(); }
        ConditionalTokenReader {matching, tag}
    }
    pub fn exclude<Tk: Token>(tokens: Vec<Tk>, nb: usize, tag: Tag) -> Self {
        let mut matching = vec![Self::success(); nb];
        for tk in tokens { matching[tk.id()] = Self::fail(); }
        ConditionalTokenReader {matching, tag}
    }
}

impl<Tk: Token> Reader<Tk> for ConditionalTokenReader<Tk> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult { success: None, ongoing: Some(this.clone()) }
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.matching[token.id()].clone()
    }
}

impl<Tk: Token> TreeBuilder for ConditionalTokenReader<Tk> {
    fn tag(&self) -> Tag {
        self.tag
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}

