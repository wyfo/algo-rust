use list::List;
use std::fmt::Debug;
use std::rc::Rc;
use traces::Trace;
use trees::*;

pub mod epsilon_reader;
pub mod list_reader;
pub mod loop_reader;
pub mod policy_reader;
pub mod ref_reader;
pub mod switch_reader;
pub mod tagger_reader;
pub mod token_reader;
pub mod conditional_token_reader;
pub mod optional_reader;
pub mod memoization;

pub type TokenId = usize;

pub trait Token: Copy + Debug {
    fn id(&self) -> TokenId;
    fn desc(&self) -> String {
        self.id().to_string().replace("\n", "\\n")
    }
}

impl Token for u8 {
    fn id(&self) -> TokenId {
        return *self as usize;
    }
}

impl Token for char {
    fn id(&self) -> TokenId {
        return *self as usize;
    }
}

impl Token for TokenId {
    fn id(&self) -> TokenId {
        *self
    }
}

#[derive(Debug, Clone)]
pub struct ReadingResult<Tk: Token> {
    pub success: Option<Rc<List<Trace>>>,
    pub ongoing: Option<Rc<dyn Reader<Tk>>>,
}

impl<Tk: Token> ReadingResult<Tk> {
    pub fn none() -> ReadingResult<Tk> {
        ReadingResult { success: None, ongoing: None }
    }
}

pub trait Reader<Tk: Token>: TreeBuilder + Debug {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk>;
    fn read(&self, this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk>;
}

pub fn epsilon<Tk: Token>(this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
    let res = this.epsilon(this);
    res
}

pub fn read<Tk: Token>(this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
    let res = this.read(this, token);
    res
}

pub fn rc_reader<Tk: Token, R: Reader<Tk> + 'static>(reader: R) -> Rc<dyn Reader<Tk>> {
    Rc::new(reader)
}


