use list::List;
use list::Stack;
use parser::parse;
use reader;
use reader::epsilon;
use reader::rc_memo_reader;
use reader::read;
use reader::Reader;
use reader::switch_reader::SwitchReader;
use std::fmt::Debug;
use std::iter::empty;
use std::rc::Rc;
use symbols::Symbol;
use traces::Policy;
use traces::Trace;
use trees::SwitchBuilder;

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
pub struct NoToken {
    start: usize,
    stop: usize,
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
                SwitchBuilder::Case(_, Some(name)) => name,
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

fn tokenize(s: &String, lexer: Rc<dyn Reader<u8>>) -> TokenIter {
    TokenIter { bytes_consumed: 0, remaining_bytes: s.as_bytes(), lexer }
}

fn tokenize_to_vec(s: &String, lexer: Rc<dyn Reader<u8>>) -> Result<Vec<Token>, NoToken> {
    let mut vec = Vec::new();
    for res_token in tokenize(s, lexer) {
        let token = res_token?;
        vec.push(token)
    }
    Ok(vec)
}
