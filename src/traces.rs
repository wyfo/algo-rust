use list::List;
use list::Stack;
use reader::*;
use std::rc::Rc;

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
    Token,
    Epsilon,
}

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
static mut _TOKEN: Option<Rc<List<Trace>>> = Option::None;
static mut _EPSILON: Option<Rc<List<Trace>>> = Option::None;

pub fn new_traces() -> Rc<List<Trace>> {
    unsafe {
        match _EMPTY_TRACES {
            None => {
                _EMPTY_TRACES = Option::Some(Rc::new(List::Nil::<Trace>));
                _EMPTY_TRACES.as_ref().unwrap().clone()
            },
            Some(ref v) => v.clone(),
        }
    }
}

pub fn token_trace() -> Rc<List<Trace>> {
    unsafe {
        match _TOKEN {
            None => {
                _TOKEN = Option::Some(new_traces().push(Trace::Token));
                _TOKEN.as_ref().unwrap().clone()
            },
            Some(ref v) => v.clone(),
        }
    }
}

pub fn epsilon_trace() -> Rc<List<Trace>> {
    unsafe {
        match _EPSILON {
            None => {
                _EPSILON = Option::Some(new_traces().push(Trace::Epsilon));
                _EPSILON.as_ref().unwrap().clone()
            },
            Some(ref v) => v.clone(),
        }
    }
}
