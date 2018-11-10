use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

pub type Symbol = NonZeroUsize;

type InternStrRef = &'static str;

pub struct SymbolTable {
    storage: Vec<Box<str>>,
    map: HashMap<InternStrRef, Symbol>,
}

impl Debug for SymbolTable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut symbols = self.map.iter().collect::<Vec<_>>().clone();
        symbols.sort_by_key(|(_, sym)| *sym);
        write!(f, "SymbolTable {{{:?}}}", symbols)
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable { storage: Vec::new(), map: HashMap::new() };
        // sentinel in order to not have offset between symbol and storage index
        table.storage.push("".to_string().into_boxed_str());
        table
    }

    pub fn get(&mut self, s: &str) -> Symbol {
        match self.map.get(s) {
            Some(&sym) => sym,
            None => self.intern(s),
        }
    }

    fn intern(&mut self, s: &str) -> Symbol {
        // self.storage.len() > 0 because of sentinel
        let sym = unsafe { Symbol::new_unchecked(self.storage.len()) };
        let stored: Box<str> = s.to_string().into_boxed_str();
        let intern_ref = unsafe { &*(&*stored as *const str) };
        self.storage.push(stored);
        self.map.insert(intern_ref, sym);
        sym
    }

    pub fn val(&self, sym: Symbol) -> String {
        self.storage[sym.get()].to_string()
    }
}

pub type Tag = Option<Symbol>;

