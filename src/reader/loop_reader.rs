use list::*;
use reader::*;
use reader::policy_reader::*;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::iter::repeat;
use std::rc::Rc;
use symbols::Tag;
use traces::*;
use trees::*;

#[derive(Copy, Clone, Debug)]
pub enum LoopOrdering {
    Increasing = 1,
    Decreasing = -1,
}

pub struct LoopReader<Tk: Token> {
    stacked: StackedReader,
    pub ref_: Rc<dyn Reader<Tk>>,
    variant: Rc<dyn Reader<Tk>>,
    cursor: usize,
    policy: Policy,
    ordering: LoopOrdering,
    pub tag: Tag,
}

impl<Tk: Token> Debug for LoopReader<Tk> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}*", self.ref_)
    }
}

impl<Tk: Token> AsStackedReader<Tk> for LoopReader<Tk> {}

impl<Tk: Token + 'static> LoopReader<Tk> {
    pub fn new(ref_: Rc<dyn Reader<Tk>>, policy: Policy, ordering: LoopOrdering, tag: Tag) -> Self {
        LoopReader {
            stacked: StackedReader::without_parent(),
            ref_: ref_.clone(),
            variant: ref_,
            cursor: 0,
            policy,
            ordering,
            tag,
        }
    }

    fn first_variant(&self) -> Rc<dyn Reader<Tk>> {
        epsilon(&self.ref_).ongoing.unwrap()
    }

    fn shift(&self, this: &Rc<dyn Reader<Tk>>, traces: Rc<List<Trace, TraceEnding>>) -> Rc<dyn Reader<Tk>> {
        rc_reader(LoopReader {
            stacked: StackedReader::new(Self::as_stacked_reader(this), traces.clone()),
            ref_: self.ref_.clone(),
            variant: self.first_variant(),
            cursor: self.cursor + 1,
            policy: self.policy,
            ordering: self.ordering,
            tag: self.tag,
        })
    }

    fn replace(&self, _: &Rc<dyn Reader<Tk>>, ongoing: Rc<dyn Reader<Tk>>) -> Rc<dyn Reader<Tk>> {
        rc_reader(LoopReader {
            stacked: self.stacked.clone(),
            ref_: self.ref_.clone(),
            variant: ongoing,
            cursor: self.cursor,
            policy: self.policy,
            ordering: self.ordering,
            tag: self.tag,
        })
    }
}

impl<Tk: Token + 'static> Reader<Tk> for LoopReader<Tk> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        ReadingResult { success: Some(epsilon_trace().push(Trace::Switch(0, self.policy))), ongoing: Some(self.replace(this, self.first_variant())) }
    }

    fn read(&self, this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        let ReadingResult { success, ongoing } = read(&self.variant, token);
        let success_trace = success.clone();
        let success = success.map(|s| self.shift(this, s));
        let ongoing = ongoing.map(|o| self.replace(this, o));
        let ongoing = LoopPolicyReader::of(success.clone(), ongoing, success_trace, self.cursor + 1);
        ReadingResult {
            success: success.map(|success| stacked_trace().push(Trace::Tmp(Self::as_stacked_reader(&(success as Rc<dyn Reader<Tk>>)))).push(Trace::Switch(self.cursor * (self.ordering as usize), self.policy))),
            ongoing,
        }
    }
}

impl<Tk: Token> TreeBuilder for LoopReader<Tk> {
    fn tag(&self) -> Tag {
        self.tag
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        SwitchBuilder::Loop
    }

    fn node_builder(&self) -> NodeBuilder {
        (Box::new(repeat(self.ref_.as_ref().as_tree_builder())), self.tag)
    }
}
