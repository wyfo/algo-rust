use list::List;
use std::rc::Rc;
use traces::*;
use trees::TreeBuilder;
use std::fmt::Debug;
use symbols::Tag;
use trees::LeafBuilder;
use trees::SwitchBuilder;
use trees::NodeBuilder;
use trees::VolatileBuilder;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::any::Any;

pub mod epsilon_reader;
pub mod list_reader;
pub mod loop_reader;
pub mod policy_reader;
pub mod ref_reader;
pub mod switch_reader;
pub mod tagger_reader;
pub mod token_reader;
pub mod conditional_token_reader;

pub type TokenId = usize;

pub trait Token: Copy + Debug {
    fn id(&self) -> TokenId;
}

impl Token for u8 {
    fn id(&self) -> TokenId {
        return *self as usize;
    }
}

impl Token for usize {
    fn id(&self) -> TokenId {
        return *self;
    }
}

impl Token for char {
    fn id(&self) -> TokenId {
        return *self as usize;
    }
}

#[derive(Debug, Clone)]
pub struct ReadingResult<Tk: Token> {
    pub success: Option<Rc<List<Trace>>>,
    pub ongoing: Option<Rc<dyn Reader<Tk>>>,
}

pub trait Reader<Tk: Token>: TreeBuilder + AsAny + Debug {
    fn tag(&self) -> Tag;
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk>;
    fn read(&self, this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk>;
}

pub fn epsilon<Tk: Token>(this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
    this.epsilon(this)
}

pub fn read<Tk: Token>(this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
    this.read(this, token)
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Memoized<Tk: Token, R: Reader<Tk>> {
    eps: RefCell<Option<ReadingResult<Tk>>>,
    reads: Vec<RefCell<Option<ReadingResult<Tk>>>>,
    reader: Rc<dyn Reader<Tk>>,
    phantom: PhantomData<R>,
}

impl<Tk: Token, R: Reader<Tk> + 'static> Memoized<Tk, R> {
    fn new(reader: R, n: usize) -> Self {
        Memoized {
            eps: RefCell::new(None),
            reads: vec![RefCell::new(None); n],
            reader: Rc::new(reader),
            phantom: PhantomData,
        }
    }
}

impl<Tk: Token + 'static, R: Reader<Tk> + 'static> AsAny for Memoized<Tk, R> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<Tk: Token, R: Reader<Tk>> TreeBuilder for Memoized<Tk, R> {
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
    fn tag(&self) -> Tag {
        self.reader.tag()
    }

    fn epsilon(&self, this: &Rc<Reader<Tk>>) -> ReadingResult<Tk> {
        {
            let mut eps = self.eps.borrow_mut();
            if eps.is_some() {
                return eps.as_ref().unwrap().clone();
            }
            *eps = match self.reader.as_any().downcast_ref::<R>() {
                Some(reader) => Some(reader.epsilon(&self.reader)),
                None => unimplemented!()
            }
        }
        self.epsilon(this)
    }

    fn read(&self, this: &Rc<Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        {
            let mut res = self.reads[token.id() as usize].borrow_mut();
            if res.is_some() {
                return res.as_ref().unwrap().clone();
            }
            *res = match self.reader.as_any().downcast_ref::<R>() {
                Some(reader) => Some(reader.read(&self.reader, token)),
                None => unimplemented!()
            }
        }
        self.read(this, token)
    }
}

pub fn rc_reader<Tk: Token, R: Reader<Tk> + 'static>(reader: R) -> Rc<dyn Reader<Tk>> {
    Rc::new(reader)
}

pub fn rc_memo_reader<Tk: Token + 'static, R: Reader<Tk> + 'static>(reader: R, n: usize) -> Rc<dyn Reader<Tk>> {
    Rc::new(Memoized::new(reader, n))
}

