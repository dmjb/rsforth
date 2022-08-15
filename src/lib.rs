#![feature(const_mut_refs)]
use std::collections::HashMap;
use std::rc::Rc;

pub struct State {
    pub stack: Stack,
    pub is_compiling: bool,
    pub input: Input,
    pub words: HashMap<String, Rc<Word>>
}

impl State {
    pub fn new() -> State {
        State {
            stack: Vec::new(),
            is_compiling: false,
            input: Input::new(),
            words: HashMap::new()
        }
    }

    pub fn pop(&mut self) -> i64 {
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, val: i64) -> () {
        self.stack.push(val);
    }

    pub fn get_word(&self, name: &str) -> Option<Rc<Word>> {
        self.words.get(name).map(|r| Rc::clone(r))
    }

    pub fn put_word(&mut self, word: &Word) -> () {
        let name = match &word.name {
            WordName::Dynamic(s) => s.clone(),
            WordName::Static(s) => String::from(*s)
        };
        self.words.insert(name, Rc::new(word.clone()));
    }
}

type Stack = Vec<i64>;
type StkFn = fn(&mut State) -> ();


#[derive(Clone)]
pub enum WordOp {
    Primitive(StkFn),
    Composite(Vec<WordOp>),
    Create(i64)
}

#[derive(Clone, Debug)]
pub enum WordName {
    Dynamic(String),
    Static(&'static str)
}

impl WordOp {
    pub fn do_op(&self, state: &mut State) -> () {
        match self {
            WordOp::Create(i) => state.push(i as *const _ as i64),
            WordOp::Primitive(f) => f(state),
            WordOp::Composite(w) => w.iter().for_each(|word| word.do_op(state))
        }
    }
}

#[derive(Clone)]
pub struct Word {
    pub name: WordName,
    pub op: WordOp
}

impl Word {
    pub const fn new_dynamic(name: String, op: Vec<WordOp>) -> Word {
        Word {
            name: WordName::Dynamic(name),
            op: WordOp::Composite(op)
        }
    }

    pub const fn new_primitive(name: &'static str, op: StkFn) -> Word {
        Word {
            name: WordName::Static(name),
            op: WordOp::Primitive(op)
        }
    }

    pub fn run(&self, state: &mut State) -> () {
        self.op.do_op(state);
    }
}

use std::io;

pub struct Input {
    buffer: Vec<String>,
    index: usize
}

impl Input {
    pub fn get_next_word(&mut self) -> String {
        match self.get_next_word_from_buffer() {
            Some(word) => word,
            None => self.new_buffer()
        }
    }

    pub fn get_next_word_from_buffer(&mut self) -> Option<String> {
        if self.index >= self.buffer.len() {
            Option::None
        } else {
            let res = Option::from(self.buffer[self.index].clone());
            self.index += 1;
            res
        }
    }

    pub fn new() -> Input {
        Input {
            buffer: Vec::new(),
            index: 0usize
        }
    }

    fn new_buffer(&mut self) -> String {
        let mut raw_input = String::new();
        io::stdin().read_line(&mut raw_input).unwrap();
        if (raw_input.len() != 0) {
            // knock newline off end
            raw_input.truncate(raw_input.len() - 1);
            raw_input = raw_input.to_ascii_lowercase();
            self.buffer = raw_input.split(' ').map(|s| String::from(s)).collect();
            self.index = 1;
            self.buffer[0].clone()
        } else {
            // HIDEOUS hack
            // TODO: implement proper Ctrl/D handling
            String::from("exit")
        }
    }
}