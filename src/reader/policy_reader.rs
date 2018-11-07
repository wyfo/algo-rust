use list::*;
use reader::*;
use std::rc::Rc;
use traces::Trace;
use trees::*;
use symbols::Tag;
use std::any::Any;

#[derive(Debug)]
pub struct PolicyReader<Tk: Token> {
    succeeded: Rc<dyn Reader<Tk>>,
    still_ongoing: Rc<dyn Reader<Tk>>,
    success_trace: Rc<List<Trace>>,
    trace_index: usize,
}

pub trait Decide<Tk: Token> where Self: 'static + Sized + Reader<Tk> {
    fn new(policy_reader: PolicyReader<Tk>) -> Self;

    fn of(succeeded: Option<Rc<dyn Reader<Tk>>>, still_ongoing: Option<Rc<dyn Reader<Tk>>>,
          success_trace: Option<Rc<List<Trace>>>, trace_index: usize) -> Option<Rc<dyn Reader<Tk>>> {
        if succeeded.is_some() && still_ongoing.is_some() {
            Some(rc_reader(Self::new(PolicyReader {
                succeeded: succeeded.unwrap(),
                still_ongoing: still_ongoing.unwrap(),
                success_trace: success_trace.unwrap(),
                trace_index,
            })))
        } else {
            succeeded.or(still_ongoing)
        }
    }
    fn policy_reader(&self) -> &PolicyReader<Tk>;

    fn between(ongoing_success: Rc<List<Trace>>, succeeded_success: Rc<List<Trace>>) -> Rc<List<Trace>>;

    fn read_and_decide(&self, token: Tk) -> ReadingResult<Tk> {
        let policy_reader = self.policy_reader();
        let ReadingResult { success: ongoing_success, ongoing: ongoing_ongoing } = read(&policy_reader.still_ongoing, token);
        let ReadingResult { success: succeeded_success, ongoing: succeeded_ongoing } = read(&policy_reader.succeeded, token);
        let ongoing = Self::of(succeeded_ongoing.clone(),
                               ongoing_ongoing,
                               Some(policy_reader.success_trace.clone()), policy_reader.trace_index);
        let success = if ongoing_success.is_some() && succeeded_success.is_some() {
            Some(Self::between(ongoing_success.unwrap(), succeeded_success.unwrap()))
        } else {
            ongoing_success.or(succeeded_success)
        };
        ReadingResult { success, ongoing }
    }
}

#[derive(Debug)]
pub struct ListPolicyReader<Tk: Token>(PolicyReader<Tk>);

impl<Tk: 'static + Token> Decide<Tk> for ListPolicyReader<Tk> {
    fn new(policy_reader: PolicyReader<Tk>) -> Self {
        ListPolicyReader(policy_reader)
    }
    fn policy_reader(&self) -> &PolicyReader<Tk> {
        &self.0
    }

    fn between(_: Rc<List<Trace>>, _: Rc<List<Trace>>) -> Rc<List<Trace>> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct LoopPolicyReader<Tk: Token>(PolicyReader<Tk>);

impl<Tk: 'static + Token> Decide<Tk> for LoopPolicyReader<Tk> {
    fn new(policy_reader: PolicyReader<Tk>) -> Self {
        LoopPolicyReader(policy_reader)
    }
    fn policy_reader(&self) -> &PolicyReader<Tk> {
        &self.0
    }

    fn between(_: Rc<List<Trace>>, _: Rc<List<Trace>>) -> Rc<List<Trace>> {
        unimplemented!()
    }
}

impl<Tk: 'static + Token> Reader<Tk> for ListPolicyReader<Tk> {
    fn tag(&self) -> Tag {
        None
    }

    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        unimplemented!()
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.read_and_decide(token)
    }
}

impl<Tk: 'static + Token> Reader<Tk> for LoopPolicyReader<Tk> {
    fn tag(&self) -> Tag {
        None
    }

    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        unimplemented!()
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.read_and_decide(token)
    }
}

impl<Tk: Token> TreeBuilder for ListPolicyReader<Tk> {
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

impl<Tk: Token> TreeBuilder for LoopPolicyReader<Tk> {
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
