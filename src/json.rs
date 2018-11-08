use reader::Reader;
use std::rc::Rc;
use lexer;
use reader::Token;
use reader::rc_reader;
use reader::ref_reader::RefReader;
use reader::token_reader::TokenReader;
use symbols::SymbolTable;

fn json_grammar(table: &mut SymbolTable) -> (Rc<dyn Reader<usize>>, Rc<dyn Reader<&'static lexer::Token>>) {
    let tag = |s| Some(table.get(s));
    let char_reader = |c: char| rc_reader(TokenReader {token_ref: c.id(), tag: tag(&c.to_string())});

    let LEFT_BRACE = char_reader('{');
    let RIGHT_BRACE = char_reader('}');
    let COMMA = char_reader(',');
    let value = rc_reader(RefReader::new());
    (COMMA, value)
}