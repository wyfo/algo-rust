use std::rc::Rc;
use list::List;
use reader::*;

#[derive(Copy, Clone, Debug)]
pub enum Policy {
    Shortest,
    Longest,
}

#[derive(Clone, Debug)]
pub enum Trace {
    Switch(TokenId, Policy),
    Rec(Rc<List<Trace>>),
    Tmp(Rc<StackedReader>),
}

//#[derive(Clone)]
//pub struct StackedReader {
//    pub parent: Option<Rc<StackedReader>>,
//    pub prev_sucess: Option<Rc<List<Trace>>>,
//}

pub type StackedReader = List<Rc<List<Trace>>>;

impl StackedReader {
    pub fn without_parent() -> Self {
        List::Nil
    }
    pub fn new(parent: Rc<StackedReader>, prev_success: Rc<List<Trace>>) -> Self {
        List::Cons(prev_success, parent)
    }
}

pub trait AsStackedReader<Tk: Token> {
    fn as_stacked_reader(this: &Rc<dyn Reader<Tk>>) -> Rc<StackedReader> {
        unsafe {
            Rc::from_raw(Rc::into_raw(this.clone()) as *const StackedReader)
        }
    }
}

static mut _EMPTY_TRACES: Option<Rc<List<Trace>>> = Option::None;

pub fn new_traces() -> Rc<List<Trace>> {
    unsafe {
        match _EMPTY_TRACES {
            None => {
                _EMPTY_TRACES = Option::Some(Rc::new(List::Nil::<Trace>));
                new_traces()
            },
            Some(ref v) => v.clone(),
        }
    }
}
