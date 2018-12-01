use list::*;
use reader::*;
use reader::policy_reader::*;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use symbols::Tag;
use traces::*;
use trees::*;

pub struct ListReader<Tk: Token> {
    stacked: StackedReader,
    pub elts: Rc<Vec<Rc<dyn Reader<Tk>>>>,
    cur_elt: Option<Rc<dyn Reader<Tk>>>,
    cursor: usize,
    pub tag: Tag,
}

impl<Tk: Token> Debug for ListReader<Tk> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}{}", self.elts, if self.cursor == self.elts.len() {"".to_string()} else {format!(":{}", self.cursor)})
    }
}

impl<Tk: Token> AsStackedReader<Tk> for ListReader<Tk> {}

impl<Tk: Token + 'static> ListReader<Tk> {
    pub fn new(elts: Vec<Rc<dyn Reader<Tk>>>, tag: Tag) -> Self {
        let first = elts[0].clone();
        ListReader {
            stacked: StackedReader::without_parent(),
            elts: Rc::new(elts),
            cur_elt: Some(first),
            cursor: 0,
            tag,
        }
    }

    fn shift(&self, this: &Rc<dyn Reader<Tk>>, traces: Rc<List<Trace, TraceEnding>>) -> Rc<dyn Reader<Tk>> {
        let cursor = self.cursor + 1;
        let cur_elt = if cursor == self.elts.len() {
            None
        } else {
            Some(self.elts[cursor].clone())
        };
        let stacked = StackedReader::new(Self::as_stacked_reader(this), traces.clone());
        rc_reader(ListReader {
            stacked,
            elts: self.elts.clone(),
            cur_elt,
            cursor,
            tag: self.tag,
        })
    }

    fn replace(&self, _: &Rc<dyn Reader<Tk>>, ongoing: Rc<dyn Reader<Tk>>) -> Rc<dyn Reader<Tk>> {
        rc_reader(ListReader {
            stacked: self.stacked.clone(),
            elts: self.elts.clone(),
            cur_elt: Some(ongoing),
            cursor: self.cursor,
            tag: self.tag,
        })
    }

    fn process(&self, this: &Rc<dyn Reader<Tk>>, to_res: impl Fn(&Rc<dyn Reader<Tk>>) -> ReadingResult<Tk>) -> ReadingResult<Tk> {
        let ReadingResult { success, ongoing } = to_res(self.cur_elt.as_ref().unwrap());
        let success_trace = success.clone();
        let success = success.map(|s| self.shift(this, s));
        let ongoing = ongoing.map(|o| self.replace(this, o));
        if let Some(success) = success {
            if self.cursor + 1 == self.elts.len() {
                ReadingResult { success: Some(stacked_trace().push(Trace::Tmp(Self::as_stacked_reader(&(success as Rc<dyn Reader<Tk>>))))), ongoing }
            } else {
                let ReadingResult { success: forward_success, ongoing: forward_ongoing } = epsilon(&success);
                let forward_ongoing = ListPolicyReader::of(forward_ongoing, ongoing, success_trace, self.cursor);
                ReadingResult { success: forward_success, ongoing: forward_ongoing }
            }
        } else {
            ReadingResult { success: None, ongoing }
        }
    }
}

impl<Tk: Token + 'static> Reader<Tk> for ListReader<Tk> {
    fn epsilon(&self, this: &Rc<dyn Reader<Tk>>) -> ReadingResult<Tk> {
        self.process(this, epsilon)
    }

    fn read(&self, this: &Rc<dyn Reader<Tk>>, token: Tk) -> ReadingResult<Tk> {
        self.process(this, |r| read(r, token))
    }
}

impl<Tk: Token + 'static> TreeBuilder for ListReader<Tk> {
    fn tag(&self) -> Tag {
        self.tag
    }

    fn switch_builder(&self, _: usize) -> SwitchBuilder {
        unimplemented!()
    }

    fn node_builder(&self) -> NodeBuilder {
        (Box::new(self.elts.iter().map(|elt| elt.as_tree_builder())), self.tag)
    }
}
