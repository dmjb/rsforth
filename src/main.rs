#![feature(const_mut_refs)]

mod lib;
use lib::{State, Word};

inventory::collect!(Word);

fn main() {
    let mut state = State::new(); 

    for word in inventory::iter::<Word> {
        state.put_word(word)
    }

    loop {
        let next_word = state.input.get_next_word();
        if next_word.is_empty() {
            continue
        }
        match state.get_word(&next_word) {
            Some(w) => {
                let word = state.get_word(&next_word).unwrap();
                word.run(&mut state);
            },
            None => {
                match next_word.as_str() {
                    "exit" => break,
                    i => match i.parse::<i64>() {
                        Ok(n) => {
                            state.stack.push(n)
                        },
                        Err(_) => println!("Bad integer {}", i)
                    }
                }
            }
        }
    }
}

/* Word Macros */

macro_rules! stack {
    ($func_name:ident, $s:ident, $depth:expr, $fn: block) => {
      inventory::submit! {
        let fname = stringify!($func_name);
        fn $func_name($s: &mut State) -> () {
          if $s.stack.len() < $depth {
              println!("Underflow, expected {} items on the stack", $depth)
          } else {
               $fn
          }
        }
        Word::new_primitive(fname, $func_name)
      }
    };
}

macro_rules! stack_unsafe { ($func_name:ident, $s:ident, $depth:expr, $fn: block) => {
    inventory::submit! {
        let fname = stringify!($func_name);
        fn $func_name($s: &mut State) -> () {
            if $s.stack.len() < $depth {
                println!("Underflow, expected {} items on the stack", $depth)
            } else {
                unsafe {
                    $fn
                }
            }
        }
        Word::new_primitive(fname, $func_name)
      }
    };
}

macro_rules! binop {
    ($func_name:ident, $op:tt) => {
        stack!($func_name, s, 2, {
            let arg1 = s.pop();
            let arg2 = s.pop();
            s.push(arg1 $op arg2)
        });
    }
}

macro_rules! binop_bool {
    ($func_name:ident, $op:tt) => {
        stack!($func_name, s, 2, {
            let arg1 = s.pop();
            let arg2 = s.pop();
            s.push((arg1 $op arg2) as i64)
        });
    }
}

macro_rules! word_binop_bool {
    ($func_name:ident, $op:tt) => {
        binop_bool!($func_name, $op);
    };
}

macro_rules! word_binop {
    ($func_name:ident, $op:tt) => {
        binop!($func_name, $op);
    };
}

macro_rules! word_unop_bool {
    ($func_name:ident, $op:expr) => {
        stack!($func_name, s, 1, {
            let $func_name = s.pop();
            s.push(($op) as i64)
        });
    };
}

macro_rules! word_unop {
    ($func_name:ident, $op:expr) => {
        stack!($func_name, s, 1, {
            let $func_name = s.pop();
            s.push($op)
        });
    };
}

/* Word defns */

stack!(peek, s, 1, { println!("{}", s.stack.last().unwrap()) });

stack!(pop, s, 1, {
    let res = s.stack.pop().unwrap();
    println!("{}", res)
});

stack!(drop, s, 1, { println!("{}, ok", s.pop()) });

stack!(dup, s, 1, { s.push(s.stack.last().unwrap().clone()) });

stack!(dupnz, s, 1, { 
    let top = s.stack.last().unwrap().clone();
    if top != 0 { s.push(top) }
});

stack!(swap, s, 2, { 
    let top = s.pop();
    let next = s.pop();
    s.push(top);
    s.push(next)
});

stack!(swap2, s, 4, { 
    let one = s.pop();
    let two = s.pop();
    let three = s.pop();
    let four = s.pop();
    s.push(three);
    s.push(four);
    s.push(one);
    s.push(two);
});

stack!(rot, s, 3, { 
    let one = s.pop();
    let two = s.pop();
    let three = s.pop();
    s.push(two);
    s.push(three);
    s.push(one);
});

stack!(revrot, s, 3, { 
    let one = s.pop();
    let two = s.pop();
    let three = s.pop();
    s.push(three);
    s.push(one);
    s.push(two);
});

stack!(over, s, 2, { 
    let second_last = s.stack[s.stack.len() - 2];
    s.push(second_last);
});

stack!(dup2, s, 2, { 
    let one = s.pop();
    let two = s.pop();
    s.push(one);
    s.push(two);
});

stack!(drop2, s, 2, { 
    s.pop();
    s.pop();
});

stack_unsafe!(store, s, 1, { 
    let ptr = s.stack.pop().unwrap() as *mut i64;
    *ptr = s.stack.pop().unwrap();
});

stack_unsafe!(load, s, 1, { 
    let ptr = s.stack.pop().unwrap() as *mut i64;
    s.stack.push(*ptr);
});

stack_unsafe!(cstore, s, 1, { 
    let ptr = s.pop() as *mut u8;
    *ptr = s.pop() as u8;
});

stack_unsafe!(cload, s, 1, { 
    let ptr = s.pop() as *mut i8;
    s.stack.push(*ptr as i64);
});

stack_unsafe!(ptrinc, s, 1, { 
    let ptr = s.pop() as *mut i64;
    *ptr += s.pop();
});

stack_unsafe!(ptrdec, s, 1, { 
    let ptr = s.pop() as *mut i64;
    *ptr -= s.pop();
});

word_binop_bool!(equals, ==);
word_binop_bool!(nequals, !=);
word_binop_bool!(gt, >);
word_binop_bool!(gte, >=);
word_binop_bool!(lt, <);
word_binop_bool!(lte, <=);
word_binop!(add, +);
word_binop!(sub, -);
word_binop!(div, /);
word_binop!(modu, %);
word_binop!(mul, *);
word_binop!(and, &);
word_binop!(or, |);
word_binop!(xor, ^);
word_unop_bool!(nez, nez != 0);
word_unop_bool!(ez, ez == 0);
word_unop_bool!(ltz, ltz < 0);
word_unop_bool!(gtz, gtz > 0);
word_unop_bool!(gtez, gtez >= 0);
word_unop_bool!(ltez, ltez <= 0);
word_unop!(inv, !inv);
word_unop!(neg, -neg);
word_unop!(incr, incr + 1);
word_unop!(decr, decr - 1);
