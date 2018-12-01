use list::List;
use list::Stack;
use reader::*;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum Policy {
    Shortest,
    Longest,
}

#[derive(Copy, Clone, Debug)]
pub enum TraceEnding {
    Token,
    Epsilon,
    Stacked,
}

#[derive(Clone, Debug)]
pub enum Trace {
    Switch(TokenId, Policy),
    Rec(Rc<List<Trace, TraceEnding>>),
    Tmp(Rc<StackedReader>),
}

pub type StackedReader = List<Rc<List<Trace, TraceEnding>>, ()>;

impl StackedReader {
    pub fn without_parent() -> Self {
        List::Nil(())
    }
    pub fn new(parent: Rc<StackedReader>, prev_success: Rc<List<Trace, TraceEnding>>) -> Self {
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

static mut _TOKEN: Option<Rc<List<Trace, TraceEnding>>> = Option::None;
static mut _EPSILON: Option<Rc<List<Trace, TraceEnding>>> = Option::None;
static mut _STACKED: Option<Rc<List<Trace, TraceEnding>>> = Option::None;

pub fn token_trace() -> Rc<List<Trace, TraceEnding>> {
    unsafe {
        match _TOKEN {
            None => {
                _TOKEN = Option::Some(Rc::new(List::Nil(TraceEnding::Token)));
                _TOKEN.as_ref().unwrap().clone()
            }
            Some(ref v) => v.clone(),
        }
    }
}

pub fn epsilon_trace() -> Rc<List<Trace, TraceEnding>> {
    unsafe {
        match _EPSILON {
            None => {
                _EPSILON = Option::Some(Rc::new(List::Nil(TraceEnding::Epsilon)));
                _EPSILON.as_ref().unwrap().clone()
            }
            Some(ref v) => v.clone(),
        }
    }
}

pub fn stacked_trace() -> Rc<List<Trace, TraceEnding>> {
    unsafe {
        match _STACKED {
            None => {
                _STACKED = Option::Some(Rc::new(List::Nil(TraceEnding::Stacked)));
                _STACKED.as_ref().unwrap().clone()
            }
            Some(ref v) => v.clone(),
        }
    }
}
