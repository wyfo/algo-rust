use reader;
use reader::Reader;
use std::rc::Rc;
use list::List;
use traces::Trace;
use reader::epsilon;
use reader::read;

pub struct ParsingResult {
    pub success: Option<Rc<List<Trace>>>,
    pub success_len: usize,
    pub nb_tokens_read: usize,
}

impl ParsingResult {
    fn is_complete(&self) -> bool {
        self.success_len == self.nb_tokens_read
    }
}

pub fn parse<Tk: reader::Token>(tokens: impl IntoIterator<Item=Tk>, reader: &Rc<dyn Reader<Tk>>) -> ParsingResult {
    let eps = epsilon(reader);
    let mut reader = eps.ongoing;
    let mut success = eps.success;
    let mut success_len = 0;
    let mut nb_tokens_read = 0;
    for tk in tokens {
        nb_tokens_read += 1;
        if reader.is_none() { break; }
        let res = read(reader.as_ref().unwrap(), tk);
        if res.success.is_some() {
            success = res.success;
            success_len = nb_tokens_read;
        }
        reader = res.ongoing;
    }
    ParsingResult { success, success_len, nb_tokens_read }
}
