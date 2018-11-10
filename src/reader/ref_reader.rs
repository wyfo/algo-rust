use reader::*;
use std::rc::Rc;
use trees::*;
use symbols::Tag;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;

pub struct RefReader<Tk: Token> {
    pub val: Option<Rc<dyn Reader<Tk>>>
}

impl<Tk: Token> Debug for RefReader<Tk> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "@")
    }
}

impl<Tk: Token> RefReader<Tk> {
    pub fn new() -> Self {
        RefReader {val: None}
    }
    pub fn set(this: Rc<dyn Reader<Tk>>, val: Rc<dyn Reader<Tk>>) -> Rc<dyn Reader<Tk>> {
        unsafe {
            let reader = &mut *(this.as_ref() as *const _  as *mut RefReader<Tk>);
            reader.val = Some(val);
            this
        }
    }
}

impl<Tk: Token + 'static> Reader<Tk> for RefReader<Tk> {
    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        epsilon(self.val.as_ref().unwrap())
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, _: Tk) -> ReadingResult<Tk> {
        unimplemented!()
    }
}

impl<Tk: Token> TreeBuilder for RefReader<Tk> {
    fn tag(&self) -> Tag {
        self.val.as_ref().unwrap().tag()
    }

    fn is_volatile(&self) -> VolatileBuilder {
        Some((self.val.as_ref().unwrap().as_tree_builder(), None))
    }

    fn leaf_builder(&self) -> LeafBuilder {
        unimplemented!()
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}

