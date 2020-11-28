use std::io;
use std::collections::HashMap;

type Stack = Vec<i64>;
type StkFn = fn(&mut Stack) -> ();

fn main() {
    let mut stack = Vec::new();

    let mut input = String::new();

    let mut word_map: HashMap<String, StkFn> = HashMap::new();

    let mut is_compiling = false;

    let words: Vec<Word> = vec!(
        Word {
            name: String::from("+"),
            op: add
        },
        Word {
            name: String::from("-"),
            op: sub
        },
        Word {
            name: String::from("*"),
            op: mul
        },
        Word {
            name: String::from("/"),
            op: div
        },
        Word {
            name: String::from("."),
            op: peek
        }
    );

    for word in words {
        word_map.insert(word.name, word.op);
    }

    loop {
        io::stdin().read_line(&mut input).unwrap();
        // knock newline off end
        input.truncate(input.len() - 1);
        input = input.to_ascii_lowercase();
        let istr = input.as_str();
        if word_map.contains_key(&input) {
            word_map.get(istr).unwrap()(&mut stack)
        } else {
            match istr {
                "exit" => break,
                i => match i.parse::<i64>() {
                    Ok(n) => {
                        stack.push(n)
                    },
                    Err(_) => println!("Bad integer {}", i),
                }
            };
        }
        input.clear();
    }
}

struct Word {
    name: String,
    op: fn(&mut Stack) -> ()
}

fn peek(s: &mut Stack) -> () {
    stack(s, 1, |st| println!("{}", st.last().unwrap()))
}

fn stack<F>(s: &mut Stack, depth: usize, f: F) -> () where F: Fn(&mut Stack) -> () {
   if s.len() < depth {
        println!("Underflow, expected {} items on the stack", depth)
   } else {
        f(s);
   } 
}

macro_rules! stack {
    ($func_name:ident, $s:ident, $depth:expr, $fn: block) => {
       fn $func_name($s: &mut Stack) -> () {
           if $s.len() < $depth {
              println!("Underflow, expected {} items on the stack", $depth)
           } else {
               $fn
           }
       } 
    };
}

macro_rules! stack_unsafe {
    ($func_name:ident, $s:ident, $depth:expr, $fn: block) => {
       fn $func_name($s: &mut Stack) -> () {
           if $s.len() < $depth {
              println!("Underflow, expected {} items on the stack", $depth)
           } else {
               unsafe {
                   $fn
               }
           }
       } 
    };
}

fn binop<F>(s: &mut Stack, f: F) -> () where F: Fn(i64, i64) -> i64 {
    stack(s, 2, |s| {
        let arg1 = s.pop().unwrap();
        let arg2 = s.pop().unwrap();
        s.push(f(arg1, arg2))
    })
}

fn binop_bool<F>(s: &mut Stack, f: F) -> () where F: Fn(i64, i64) -> bool {
    stack(s, 2, |s| {
        let arg1 = s.pop().unwrap();
        let arg2 = s.pop().unwrap();
        s.push(f(arg1, arg2) as i64)
    })
}

macro_rules! word_binop_bool {
    ($func_name:ident, $op:tt) => {
       fn $func_name(s: &mut Stack) -> () {
           binop_bool(s, |o1, o2| o1 $op o2)
       } 
    };
}

macro_rules! word_binop {
    ($func_name:ident, $op:tt) => {
       fn $func_name(s: &mut Stack) -> () {
           binop(s, |o1, o2| o1 $op o2)
       } 
    };
}

macro_rules! word_unop_bool {
    ($func_name:ident, $op:expr) => {
        fn $func_name(s: &mut Stack) -> () {
            stack(s, 1, |s| {
                let $func_name = s.pop().unwrap();
                s.push($op as i64)
            })
        }
    };
}

macro_rules! word_unop {
    ($func_name:ident, $op:expr) => {
        fn $func_name(s: &mut Stack) -> () {
            stack(s, 1, |s| {
                let $func_name = s.pop().unwrap();
                s.push($op)
            })
        }
    };
}

/*fn drop(s: &mut Stack) -> () {
    match s.pop() {
        None => println!("Stack underflow!"),
        Some(v) => println!("{}, ok", v)
    }
}*/

stack!(drop, s, 1, { println!("{}, ok", s.pop().unwrap()) });

stack!(dup, s, 1, { s.push(s.last().unwrap().clone()) });

stack!(dupnz, s, 1, { 
    let top = s.last().unwrap().clone();
    if top != 0 { s.push(top) }
});

stack!(swap, s, 2, { 
    let top = s.pop().unwrap();
    let next = s.pop().unwrap();
    s.push(top);
    s.push(next);
});

stack!(swap2, s, 4, { 
    let one = s.pop().unwrap();
    let two = s.pop().unwrap();
    let three = s.pop().unwrap();
    let four = s.pop().unwrap();
    s.push(three);
    s.push(four);
    s.push(one);
    s.push(two);
});

stack!(rot, s, 3, { 
    let one = s.pop().unwrap();
    let two = s.pop().unwrap();
    let three = s.pop().unwrap();
    s.push(two);
    s.push(three);
    s.push(one);
});

stack!(revrot, s, 3, { 
    let one = s.pop().unwrap();
    let two = s.pop().unwrap();
    let three = s.pop().unwrap();
    s.push(three);
    s.push(one);
    s.push(two);
});

stack!(over, s, 2, { 
    let second_last = s[s.len() - 2];
    s.push(second_last);
});

stack!(dup2, s, 2, { 
    let one = s.pop().unwrap();
    let two = s.pop().unwrap();
    s.push(one);
    s.push(two);
});

stack!(drop2, s, 2, { 
    s.pop().unwrap();
    s.pop().unwrap();
});

stack_unsafe!(store, s, 1, { 
    let ptr = s.pop().unwrap() as *mut i64;
    *ptr = s.pop().unwrap();
});

stack_unsafe!(load, s, 1, { 
    let ptr = s.pop().unwrap() as *mut i64;
    s.push(*ptr);
});

stack_unsafe!(cstore, s, 1, { 
    let ptr = s.pop().unwrap() as *mut u8;
    *ptr = s.pop().unwrap() as u8;
});

stack_unsafe!(cload, s, 1, { 
    let ptr = s.pop().unwrap() as *mut i8;
    s.push(*ptr as i64);
});

stack_unsafe!(ptrinc, s, 1, { 
    let ptr = s.pop().unwrap() as *mut i64;
    *ptr += s.pop().unwrap();
});

stack_unsafe!(ptrdec, s, 1, { 
    let ptr = s.pop().unwrap() as *mut i64;
    *ptr -= s.pop().unwrap();
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
