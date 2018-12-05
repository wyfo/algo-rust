use lexer;
use parser;
use reader::conditional_token_reader::ConditionalTokenReader;
use reader::epsilon_reader::EpsilonReader;
use reader::list_reader::ListReader;
use reader::loop_reader::LoopOrdering;
use reader::loop_reader::LoopReader;
use reader::memoization::rc_memo_reader;
use reader::optional_reader::OptionalReader;
use reader::rc_reader;
use reader::Reader;
use reader::ref_reader::RefReader;
use reader::switch_reader::SwitchReader;
use reader::Token;
use reader::token_reader::TokenReader;
use reader::TokenId;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use symbols::SymbolTable;
use symbols::Tag;
use traces::Policy;
use trees::Tree;
use trees::tree_from_trace;
use reader::switch_reader::MemoSwitchReader;

fn tag(table: &mut SymbolTable, s: &str) -> Tag { Some(table.get(s)) }

fn char_reader(c: char) -> Rc<dyn Reader<u8>> {
    rc_reader(TokenReader { token_ref: c.id(), tag: None })
//    rc_reader(ConditionalTokenReader::include(vec![c.id() as u8], 256, None))
}

fn char_reader2(table: &mut SymbolTable, c: char) -> Rc<dyn Reader<u8>> {
    rc_reader(TokenReader { token_ref: c.id(), tag: tag(table, &c.to_string()) })
//    rc_reader(ConditionalTokenReader::include(vec![c.id() as u8], 256, tag(table, &c.to_string())))
}

fn str_reader(table: &mut SymbolTable, s: &str) -> Rc<dyn Reader<u8>> {
    let elts = s.chars().map(|c| char_reader(c)).collect();
    rc_memo_reader(ListReader::new(elts, tag(table, s)), 256)
}

fn opt_reader<Tk: Token + 'static>(reader: Rc<dyn Reader<Tk>>) -> Rc<dyn Reader<Tk>> {
//    rc_memo_reader(SwitchReader::new(vec![rc_reader(EpsilonReader), reader], Policy::Longest, None), nb)
    rc_reader(OptionalReader::new(reader))
}

fn token_reader(token: Rc<dyn Reader<u8>>, token_ids: &HashMap<*const dyn Reader<u8>, TokenId>) -> Rc<dyn Reader<&'static lexer::Token>> {
    rc_reader(TokenReader { token_ref: token_ids[&Rc::into_raw(token)], tag: None })
//    rc_reader(ConditionalTokenReader::include(vec![token_ids[&Rc::into_raw(token)] as u8], 256, None))
}

fn json_grammar(table: &mut SymbolTable) -> (Rc<dyn Reader<u8>>, Rc<dyn Reader<&'static lexer::Token>>) {
    let LEFT_BRACE = char_reader2(table, '{');
    let RIGHT_BRACE = char_reader2(table, '}');
    let COMMA = char_reader2(table, ',');
    let COLON = char_reader2(table, ':');
    let LEFT_BRACKET = char_reader2(table, '[');
    let RIGHT_BRACKET = char_reader2(table, ']');
    let DOUBLE_QUOTE = char_reader('"');
    let MINUS = char_reader('-');
    let DOT = char_reader('.');
    let BACKSLASH = char_reader('\\');
    let TRUE = str_reader(table, "true");
    let FALSE = str_reader(table, "false");
    let NULL = str_reader(table, "null");
    let WS = rc_reader(ConditionalTokenReader::include(" \t\n\r".as_bytes().to_vec(), 256, tag(table, "WS")));
    let DIGIT = rc_reader(ConditionalTokenReader::include("0123456789".as_bytes().to_vec(), 256, None));
    let INT = rc_reader(ListReader::new(vec![
        DIGIT.clone(),
        rc_reader(LoopReader::new(DIGIT.clone(), Policy::Longest, LoopOrdering::Increasing, None)),
    ], None));
    let EXP = rc_reader(ListReader::new(vec![
        rc_reader(ConditionalTokenReader::include("eE".as_bytes().to_vec(), 256, None)),
        opt_reader(rc_reader(ConditionalTokenReader::include("+-".as_bytes().to_vec(), 256, None))),
        INT.clone(),
    ], None));
    let NUMBER = rc_reader(ListReader::new(vec![
        opt_reader(MINUS),
        INT.clone(),
        opt_reader(rc_reader(ListReader::new(vec![
            DOT,
            INT.clone(),
        ], None))),
        opt_reader(EXP),
    ], tag(table, "NUMBER")));
    let HEX = rc_reader(ConditionalTokenReader::include("0123456789ABCDEFabcdef".as_bytes().to_vec(), 256, None));
    let UNICODE = rc_reader(ListReader::new(vec![char_reader('u'), HEX.clone(), HEX.clone(), HEX.clone(), HEX.clone()], None));
    let ESC = rc_reader(ListReader::new(vec![
        BACKSLASH,
        rc_reader(SwitchReader::new(vec![
            rc_reader(ConditionalTokenReader::include("\"\\nt".as_bytes().to_vec(), 256, None)),
        UNICODE,
        ], Policy::Longest, None)),
    ], None));
    let STRING = rc_reader(ListReader::new(vec![
        DOUBLE_QUOTE.clone(),
        rc_reader(LoopReader::new(rc_reader(SwitchReader::new(vec![
            ESC,
            rc_reader(ConditionalTokenReader::exclude("\\\"".as_bytes().to_vec(), 256, None)),
        ], Policy::Longest, None)), Policy::Longest, LoopOrdering::Increasing, None)),
        DOUBLE_QUOTE.clone(),
    ], tag(table, "STRING")));

    let tokens = vec![
        LEFT_BRACE.clone(),
        RIGHT_BRACE.clone(),
        COMMA.clone(),
        COLON.clone(),
        LEFT_BRACKET.clone(),
        RIGHT_BRACKET.clone(),
        TRUE.clone(),
        FALSE.clone(),
        NULL.clone(),
        WS.clone(),
        NUMBER.clone(),
        STRING.clone(),
    ];
    let token_ids: HashMap<_, _> = tokens.iter().enumerate().map(|p| (Rc::into_raw(p.1.clone()), p.0 as TokenId)).collect();

    let value = rc_reader(RefReader::<&'static lexer::Token>::new());
    let array = rc_reader(ListReader::new(vec![
        token_reader(LEFT_BRACKET.clone(), &token_ids),
        opt_reader(rc_reader(ListReader::new(vec![
            value.clone(),
            rc_reader(LoopReader::new(
                rc_reader(ListReader::new(vec![
                    token_reader(COMMA.clone(), &token_ids),
                    value.clone()
                ], None)),
                Policy::Longest, LoopOrdering::Increasing, None,
            ))
        ], None))),
        token_reader(RIGHT_BRACKET.clone(), &token_ids),
    ], tag(table, "array")));
    let pair = rc_reader(ListReader::new(vec![
        token_reader(STRING.clone(), &token_ids),
        token_reader(COLON.clone(), &token_ids),
        value.clone()
    ], tag(table, "pair")));
    let obj = rc_reader(ListReader::new(vec![
        token_reader(LEFT_BRACE.clone(), &token_ids),
        opt_reader(rc_reader(ListReader::new(vec![
            pair.clone(),
            rc_memo_reader(LoopReader::new(
                rc_reader(ListReader::new(vec![
                    token_reader(COMMA.clone(), &token_ids),
                    pair.clone()
                ], None)),
                Policy::Longest, LoopOrdering::Increasing, None,
            ), tokens.len())
        ], None))),
        token_reader(RIGHT_BRACE.clone(), &token_ids),
    ], tag(table, "obj")));
    let value = RefReader::set(value, rc_reader(SwitchReader::new(vec![
        token_reader(STRING.clone(), &token_ids),
        token_reader(NUMBER.clone(), &token_ids),
        obj,
        array,
        token_reader(TRUE.clone(), &token_ids),
        token_reader(FALSE.clone(), &token_ids),
        token_reader(NULL.clone(), &token_ids),
    ], Policy::Longest, tag(table, "value"))));

    let lexer = rc_memo_reader(MemoSwitchReader::new(tokens, Policy::Longest, None), 256);
    let json = value.clone();
    (lexer, json)
}

pub fn tokenize_to_vec(s: &String, lexer: Rc<dyn Reader<u8>>, table: &mut SymbolTable) -> Result<Vec<Rc<lexer::Token>>, lexer::NoToken> {
    let ws = table.get("WS");
    let mut vec = Vec::new();
    for res_token in lexer::tokenize(s, lexer) {
        let token = res_token?;
        if token.name != ws { vec.push(Rc::new(token)) }
    }
    Ok(vec)
}

pub fn parse_json(s: &String, table: &mut SymbolTable) -> Option<(Vec<Rc<lexer::Token>>, Tree<Rc<lexer::Token>>)> {
    let (lxr, prsr) = json_grammar(table);
    println!("{:?}", &table);
    let lexing_start = Instant::now();
    println!("LEXING STARTED");
    let tokens = match tokenize_to_vec(s, lxr, table) {
        Ok(tks) => tks,
        Err(no_token) => {
            println!("No token found at {} - {}", no_token.start, no_token.stop);
            return None;
        },
    };
    let parsing_start = Instant::now();
    println!("PARSING STARTED");
    let res = parser::parse(tokens.iter().map(|tk| unsafe {&*(tk.as_ref() as *const _)}), &prsr);
    let success = match res.success {
        Some(s) => s,
        None => return None,
    };
    let tree_building_start = Instant::now();
    println!("TREE BUILDING STARTED");
    println!("DONE");
    let tree = tree_from_trace(prsr.as_tree_builder(), &success, &tokens);
    let time = Instant::now();
    println!("lexing = {:?}", parsing_start.duration_since(lexing_start));
    println!("parsing = {:?}", tree_building_start.duration_since(parsing_start));
    println!("tree building = {:?}", time.duration_since(tree_building_start));
    println!("time = {:?}", time.duration_since(lexing_start));
    Some((tokens, tree))
}