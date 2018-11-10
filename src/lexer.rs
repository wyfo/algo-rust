use list::List;
use list::Stack;
use parser::parse;
use reader;
use reader::Reader;
use std::rc::Rc;
use symbols::Symbol;
use traces::Trace;
use trees::SwitchBuilder;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Clone)]
pub struct Token {
    pub name: Symbol,
    pub traces: Rc<List<Trace>>,
    pub start: usize,
    pub stop: usize,
    id: reader::TokenId,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.id)
    }
}

impl<'a> reader::Token for &'a Token {
    fn id(&self) -> reader::TokenId {
        self.id
    }

    fn desc(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct NoToken {
    pub start: usize,
    pub stop: usize,
}

pub struct TokenIter<'a> {
    bytes_consumed: usize,
    remaining_bytes: &'a [u8],
    lexer: Rc<dyn Reader<u8>>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Result<Token, NoToken>;

    fn next(&mut self) -> Option<Result<Token, NoToken>> {
        if self.remaining_bytes.len() == 0 { return None; }
        let parsing_res = parse(self.remaining_bytes.iter().cloned(), &self.lexer);
        if let Some(success) = parsing_res.success {
            let id = match success.peek() {
                Trace::Switch(id, _) => *id,
                _ => panic!()
            };
            let name = match self.lexer.as_tree_builder().switch_builder(id) {
                SwitchBuilder::Case(case, _) => case.tag().unwrap(),
                _ => panic!()
            };
            let token = Token {
                name,
                traces: success,
                start: self.bytes_consumed,
                stop: self.bytes_consumed + parsing_res.success_len,
                id,
            };
            self.bytes_consumed += parsing_res.success_len;
            self.remaining_bytes = &self.remaining_bytes[parsing_res.success_len..];
            Some(Ok(token))
        } else {
            Some(Err(NoToken { start: self.bytes_consumed, stop: self.bytes_consumed + parsing_res.nb_tokens_read }))
        }
    }
}

pub fn tokenize(s: &String, lexer: Rc<dyn Reader<u8>>) -> TokenIter {
    TokenIter { bytes_consumed: 0, remaining_bytes: s.as_bytes(), lexer }
}

