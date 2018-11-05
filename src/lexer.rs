use list::List;
use reader;
use std::rc::Rc;
use traces::Policy;
use traces::Trace;
use reader::Reader;
use reader::switch_reader::SwitchReader;
use symbols::Symbol;
use std::fmt::Debug;
use std::iter::empty;
use std::collections::HashMap;
use reader::epsilon;
use reader::read;

#[derive(Clone, Debug)]
pub struct Token {
    name: Symbol,
    traces: Rc<List<Trace>>,
    start: usize,
    stop: usize,
    id: reader::TokenId,
}

impl<'a> reader::Token for &'a Token {
    fn id(&self) -> reader::TokenId {
        self.id
    }
}

#[derive(Clone, Debug)]
struct NoToken {
    start: usize,
    stop: usize,
}

//fn tokenize(s: &String, rules: Vec<Rc<dyn Reader<u8>>>) -> impl Iterator<Item=Token<u8>> {
//    let indexes: HashMap<_, _> = rules.iter().map(|r| r.tag()).enumerate().map(|t| (t.1, t.0)).collect();
//    let lexer = SwitchReader::new(rules, Policy::Longest, None);
//    let mut bytes = s.as_bytes();
//    let index = 0;
//    let read_one_token = || {
//        let eps = epsilon(lexer);
//        let mut reader = eps.ongoing;
//        let mut traces = eps.success;
//        let mut stop_index = index;
//        let mut last_index: usize = 0;
//        for (i, b) in bytes.iter().enumerate() {
//            last_index = i;
//            if reader.is_none() { break }
//            let res = read(reader, b);
//            if let Some(t) = res.success {
//                traces = Some(t);
//                stop_index += 1;
//            }
//            reader = res.ongoing
//        }
//    };
//    while !bytes.is_empty() {
//        let (token, remain) = read_one_token();
//    }
//}

