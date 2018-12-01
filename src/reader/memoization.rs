use reader::*;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use trees::*;

pub struct Memoized<Tk: Token, R: Reader<Tk>> {
    reader: R,
    eps: Option<ReadingResult<Tk>>,
    reads: Vec<Option<ReadingResult<Tk>>>,
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
            eps: None,
            reads: vec![None; n],
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

    fn switch_builder(&self, case: usize) -> SwitchBuilder {
        self.reader.switch_builder(case)
    }

    fn node_builder(&self) -> NodeBuilder {
        self.reader.node_builder()
    }
}

impl<Tk: Token + 'static, R: Reader<Tk> + 'static> Reader<Tk> for Memoized<Tk, R> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        let eps = unsafe { &mut *(&self.eps as *const _ as *mut Option<ReadingResult<Tk>>) };
        match eps {
            Some(ref memo) => memo.clone(),
            None => {
                let tmp = self.reader.epsilon(this);
                *eps = Some(tmp.clone());
                tmp
            }
        }
    }

    fn read(&self, this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        let id = token.id();
        let mut res = unsafe { &mut *(&self.reads[id as usize] as *const _ as *mut Option<ReadingResult<Tk>>) };
        match res {
            Some(ref memo) => memo.clone(),
            None => {
                let tmp = self.reader.read(this, token);
                *res = Some(tmp.clone());
                tmp
            }
        }
    }
}

pub fn rc_memo_reader<Tk: Token + 'static, R: Reader<Tk> + 'static>(reader: R, n: usize) -> Rc<dyn Reader<Tk>> {
    Rc::new(Memoized::new(reader, n))
}

pub(super) fn rc_memo_reader_from<Tk: Token + 'static, R: Reader<Tk> + 'static>(reader: R, from: &R) -> Rc<dyn Reader<Tk>> {
    Rc::new(Memoized::new(reader, Memoized::as_memoized(from).reads.len()))
}
