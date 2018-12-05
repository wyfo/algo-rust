use itertools::Itertools;
use list::Stack;
use reader::*;
use reader::memoization::rc_memo_reader_from;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::Policy;
use traces::Trace;
use trees::*;
use reader::memoization::MemoAllocator;
use reader::memoization::WithoutMemo;
use reader::memoization::WithMemo;
use std::marker::PhantomData;

type Case<Tk> = (Rc<dyn Reader<Tk>>, usize);

pub struct SwitchReader_<Tk: Token, A: MemoAllocator> {
    pub cases: Vec<Case<Tk>>,
    policy: Policy,
    pub tag: Tag,
    phantom: PhantomData<A>
}

pub type SwitchReader<Tk> = SwitchReader_<Tk, WithoutMemo>;
pub type MemoSwitchReader<Tk> = SwitchReader_<Tk, WithMemo>;

impl<Tk: Token, A: MemoAllocator> Debug for SwitchReader_<Tk, A> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({})", self.cases.iter().map(|c| format!("{:?}", c.0)).join(" | "))
    }
}

impl<Tk: Token + 'static, A: 'static + MemoAllocator> SwitchReader_<Tk, A> {
    pub fn new(cases: Vec<Rc<dyn Reader<Tk>>>, policy: Policy, tag: Tag) -> Self {
        SwitchReader_ {
            cases: cases.iter().enumerate().map(|t| (t.1.clone(), t.0 as usize)).collect(),
            policy,
            tag,
            phantom: PhantomData
        }
    }

    fn process(&self, to_res: impl Fn(&Rc<dyn Reader<Tk>>) -> ReadingResult<Tk>) -> ReadingResult<Tk> {
        let results: Vec<(ReadingResult<Tk>, usize)> = self.cases.iter().map(|(c, i)| (to_res(c), *i)).collect();
        let ongoings: Vec<Case<Tk>> = results.iter().filter_map(|(c, i)| c.ongoing.clone().map(|o| (o, *i))).collect();
        let ongoing: Option<Rc<dyn Reader<Tk>>> = if ongoings.is_empty() {
            None
        } else {
            Some(A::rc(SwitchReader_::<Tk, A> { cases: ongoings, policy: self.policy, tag: self.tag, phantom: PhantomData }, self))
        };
        let success = results.iter().find(|(c, _)| c.success.is_some()).map(|(c, i)| (c.success.clone().unwrap(), i));
        ReadingResult {
            success: success.map(|(tr, i)| tr.push(Trace::Switch(*i as usize, self.policy))),
            ongoing,
        }
    }
}

impl<Tk: Token + 'static, A: MemoAllocator + 'static> Reader<Tk> for SwitchReader_<Tk, A> {
    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        self.process(epsilon)
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.process(|r| read(r, token))
    }
}

impl<Tk: Token, A: MemoAllocator> TreeBuilder for SwitchReader_<Tk, A> {
    fn tag(&self) -> Tag {
        self.tag
    }

    fn switch_builder(&self, case: usize) -> SwitchBuilder {
        SwitchBuilder::Case(self.cases[case].0.as_tree_builder(), self.tag)
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}
