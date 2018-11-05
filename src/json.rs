use reader::Reader;
use std::rc::Rc;
use lexer;
use reader::rc_reader;

fn json_grammar() -> (Rc<dyn Reader<usize>>, Rc<dyn Reader<lexer::Token>>) {
    panic!()
}