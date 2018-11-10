use list::List;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::*;
use trees::LeafBuilder;
use trees::NodeBuilder;
use trees::SwitchBuilder;
use trees::TreeBuilder;
use trees::VolatileBuilder;

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

pub type TokenId = usize;

pub trait Token: Copy + Debug {
    fn id(&self) -> TokenId;
    fn desc(&self) -> String;
}

impl Token for u8 {
    fn id(&self) -> TokenId {
        return *self as usize;
    }

    fn desc(&self) -> String {
        return (*self as char).to_string().replace("\n", "\\n")
    }
}

impl Token for char {
    fn id(&self) -> TokenId {
        return *self as usize;
    }

    fn desc(&self) -> String {
        return self.to_string().replace("\n", "\\n")
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
    this.epsilon(this)
}

pub fn read<Tk: Token>(this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
    this.read(this, token)
}

pub struct Memoized<Tk: Token, R: Reader<Tk>> {
    reader: R,
    eps: RefCell<Option<ReadingResult<Tk>>>,
    reads: Vec<RefCell<Option<ReadingResult<Tk>>>>,
}

impl<Tk: Token, R: Reader<Tk>> Debug for Memoized<Tk, R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.reader.fmt(f)
    }
}

impl<Tk: Token, R: Reader<Tk> + 'static> Memoized<Tk, R> {
    fn new(reader: R, n: usize) -> Self {
        Memoized {
            reader,
            eps: RefCell::new(None),
            reads: vec![RefCell::new(None); n],
        }
    }

    fn as_memoized(reader: &R) -> &Self {
        unsafe { &*(reader as *const R as *const Self) }
    }
}

impl<Tk: Token, R: Reader<Tk>> TreeBuilder for Memoized<Tk, R> {
    fn tag(&self) -> Tag {
        self.reader.tag()
    }

    fn is_volatile(&self) -> VolatileBuilder {
        self.reader.is_volatile()
    }

    fn leaf_builder(&self) -> LeafBuilder {
        self.reader.leaf_builder()
    }

    fn switch_builder(&self, case: usize) -> SwitchBuilder {
        self.reader.switch_builder(case)
    }

    fn node_builder(&self) -> NodeBuilder {
        self.reader.node_builder()
    }
}

impl<Tk: Token + 'static, R: Reader<Tk> + 'static> Reader<Tk> for Memoized<Tk, R> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        let mut eps = self.eps.borrow_mut();
        if eps.is_some() {
            return eps.as_ref().unwrap().clone();
        }
        *eps = Some(self.reader.epsilon(this));
        eps.as_ref().unwrap().clone()
    }

    fn read(&self, this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        let id = token.id();
        let mut res = self.reads[id as usize].borrow_mut();
        if res.is_some() {
            return res.as_ref().unwrap().clone();
        }
        *res = Some(self.reader.read(this, token));
        res.as_ref().unwrap().clone()
    }
}

pub fn rc_reader<Tk: Token, R: Reader<Tk> + 'static>(reader: R) -> Rc<dyn Reader<Tk>> {
    Rc::new(reader)
}

pub fn rc_memo_reader<Tk: Token + 'static, R: Reader<Tk> + 'static>(reader: R, n: usize) -> Rc<dyn Reader<Tk>> {
    Rc::new(Memoized::new(reader, n))
}

fn rc_memo_reader_from<Tk: Token + 'static, R: Reader<Tk> + 'static>(reader: R, from: &R) -> Rc<dyn Reader<Tk>> {
    Rc::new(Memoized::new(reader, Memoized::as_memoized(from).reads.len()))
}

