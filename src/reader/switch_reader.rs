use list::Stack;
use reader::*;
use std::rc::Rc;
use symbols::Tag;
use traces::Policy;
use traces::Trace;
use trees::*;
use std::any::Any;

type Case<Tk> = (Rc<dyn Reader<Tk>>, usize);

#[derive(Debug)]
pub struct SwitchReader<Tk: Token> {
    pub cases: Vec<Case<Tk>>,
    policy: Policy,
    pub tag: Tag,
}

impl<Tk: Token + 'static> SwitchReader<Tk> {
    pub fn new(cases: Vec<Rc<dyn Reader<Tk>>>, policy: Policy, tag: Tag) -> SwitchReader<Tk> {
        SwitchReader { cases: cases.iter().enumerate().map(|t| (t.1.clone(), t.0 as usize)).collect(), policy, tag }
    }

    fn process(&self, to_res: impl Fn(&Rc<dyn Reader<Tk>>) -> ReadingResult<Tk>) -> ReadingResult<Tk> {
        let results: Vec<(ReadingResult<Tk>, usize)> = self.cases.iter().map(|(c, i)| (to_res(c), *i)).collect();
        let ongoings: Vec<Case<Tk>> = results.iter().filter_map(|(c, i)| c.ongoing.clone().map(|o| (o, *i))).collect();
        let ongoing: Option<Rc<dyn Reader<Tk>>> = if ongoings.is_empty() {
            None
        } else {
            Some(Rc::new(SwitchReader::<Tk> { cases: ongoings, policy: self.policy, tag: self.tag }) as Rc<dyn Reader<Tk>>)
        };
        let success = results.iter().find(|(c, _)| c.success.is_some()).map(|(c, i)| (c.success.clone().unwrap(), i));
        ReadingResult {
            success: success.map(|(tr, i)| tr.push(Trace::Switch(*i as usize, self.policy))),
            ongoing,
        }
    }
}

impl<Tk: Token + 'static> Reader<Tk> for SwitchReader<Tk> {
    fn tag(&self) -> Tag {
        self.tag
    }

    fn epsilon(&self, _: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        self.process(epsilon)
    }

    fn read(&self, _: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.process(|r| read(r, token))
    }
}

impl<Tk: Token> TreeBuilder for SwitchReader<Tk> {
    fn leaf_builder(&self) -> LeafBuilder {
        unimplemented!()
    }

    fn switch_builder(&self, case: usize) -> SwitchBuilder {
        SwitchBuilder::Case(self.cases[case].0.as_tree_builder(), self.tag)
    }

    fn node_builder(&self) -> NodeBuilder {
        unimplemented!()
    }
}

impl<Tk: Token + 'static> AsAny for SwitchReader<Tk> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}