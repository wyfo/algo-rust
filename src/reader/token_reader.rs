use reader::*;
use std::rc::Rc;
use symbols::Tag;
use trees::*;
use traces::*;
use list::*;
use std::any::Any;

#[derive(Debug)]
pub struct TokenReader {
    pub token_ref: usize,
    pub tag: Tag,
}

impl<Tk: Token> Reader<Tk> for TokenReader {
    fn tag(&self) -> Tag {
        self.tag
    }

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

impl AsAny for TokenReader {
    fn as_any(&self) -> &dyn Any {
        self
    }
}