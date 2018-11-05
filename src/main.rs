extern crate core;


mod list;
mod reader;
mod traces;
mod symbols;
mod trees;
mod lexer;
mod parser;
mod json;

use symbols::SymbolTable;
use symbols::Symbol;
use std::rc::Rc;
use reader::token_reader::TokenReader;
use reader::*;
use list::*;
use reader::list_reader::ListReader;
use reader::loop_reader::LoopReader;
use reader::conditional_token_reader::ConditionalTokenReader;
use traces::new_traces;

fn main() {
    let nil = Rc::new(List::Nil);
    let l1 = nil.push(1);
    assert_eq!(*l1.peek(), 1);
    assert_eq!(l1[0], 1);
    let (a, b) = l1.pop();
    assert_eq!(b, 1);
    assert_eq!(Rc::into_raw(a), Rc::into_raw(nil));
    let l12 = l1.push(2);
    println!("Hello, world!");
    println!("{}", *l12.peek());
    println!("{:?}", l12);
//    let mut symbols = SymbolTable::new();
//    let s1 = symbols.get("a");
//    let s2 = symbols.get("a");
//    assert_eq!(s1, s2);
//    assert_eq!(symbols.val(s1), "a");
//    assert_eq!(symbols.val(s2), "a");
//    let s3 = symbols.get("b");
//    assert_ne!(s1, s3);
//    assert_eq!(symbols.val(s3), "b");
//    let r = Rc::new(TokenReader { token_ref: 'a'.id(), tag: None }) as Rc<dyn Reader<char>>;
//    let r = rc_reader(TokenReader { token_ref: 'a'.id(), tag: None });
    let token_reader = |c: char| rc_reader(TokenReader {token_ref: c.id(), tag: None});
    let r = token_reader('a');
    let r2 = rc_reader::<char, ConditionalTokenReader<char, Vec<ReadingResult<char>>>>(ConditionalTokenReader {tag: None, matching: vec![ReadingResult {
        success: Some(new_traces()),
        ongoing: None,
    };256]});
    let mut res = read(&r, 'a');
    println!("{:?}", &res);
    let mut tree = trees::tree_from_trace(r.as_tree_builder(), &res.success.unwrap(), &vec!['a'][..]);
    println!("{:?}", &tree);
    res = read(&r2, 'b');
    println!("{:?}", &res);
    tree = trees::tree_from_trace(r.as_tree_builder(), &res.success.unwrap(), &vec!['b'][..]);
    println!("{:?}", &tree);
//    let l = rc_reader(ListReader::new(vec![r.clone(), r.clone()], Symbol::new(1)));
    let l = rc_reader(LoopReader::new(r.clone(), traces::Policy::Longest, loop_reader::LoopOrdering::Increasing, Symbol::new(1)));
    res = read(&l, 'a');
    res = read(&res.ongoing.unwrap(), 'a');
    println!("{:?}", &res);
    tree = trees::tree_from_trace(l.as_tree_builder(), &res.success.unwrap(), &vec!['a', 'a'][..]);
    println!("{:?}", &tree);
}
