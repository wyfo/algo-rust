use reader::*;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::*;
use trees::*;

pub struct TokenReader {
    pub token_ref: TokenId,
    pub tag: Tag,
}

impl Debug for TokenReader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "'{}'", self.token_ref)
    }
}

impl<Tk: Token> Reader<Tk> for TokenReader {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult { success: None, ongoing: Some(this.clone()) }
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        ReadingResult {
            success: if self.token_ref == token.id() { Some(new_traces()) } else { None },
            ongoing: None,
        }
    }
}

impl TreeBuilder for TokenReader {
    fn tag(&self) -> Tag {
        self.tag
    }

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
