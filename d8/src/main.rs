use std::io::{stdin, BufRead};
use std::fmt;
use std::error::Error;
use std::str::FromStr;
use std::collections::HashSet;

struct State {
    pc: i32,
    a: i32
}

macro_rules! opcodes {
    ([$arg:ident, $state:ident], $($op:ident => $on_eval:block),+) => (
        #[derive(Debug)]
        struct OpParseErr{failing: String}
        impl<'a> fmt::Display for OpParseErr {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Could not parse {} as opcode", self.failing)
            }
        }
        impl<'a> Error for OpParseErr {}
        
        #[derive(Debug, Clone, Copy)]
        enum Op {
            $($op),+
        }
        impl Op {
            fn eval (&self, $arg:i32, $state:&mut State) -> () {
                match self {
                    $(Op::$op => $on_eval),+
                }
                $state.pc += 1;
            }
        }
        impl FromStr for Op {
            type Err = OpParseErr;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $(if s == stringify!($op).to_lowercase() {return Ok(Op::$op)})+
                    Err(Self::Err{failing: String::from(s)})
            }
        }
    )
}

opcodes!{
    [arg, state],
    Nop => {()},
    Acc => {state.a += arg;},
    Jmp => {state.pc += arg - 1;}
}

#[derive(Debug, Clone)]
struct Instr {
    op: Op,
    arg: i32
}

impl Instr {
    fn eval(&self, state: &mut State) -> () {
        self.op.eval(self.arg, state);
    }
}

enum TermState {Bounds, Loop}
fn exec(prog: &[Instr], state: &mut State) -> TermState{
    let mut visited: HashSet<i32> = HashSet::new();
    while (state.pc as usize) < prog.len() && state.pc >= 0 {
        if visited.contains(&state.pc) {
            return TermState::Loop;
        } else {
            visited.insert(state.pc);
            prog[state.pc as usize].eval(state);
        }
    }
    TermState::Bounds
}


fn main() {
    let prog: Vec<_> = stdin().lock().lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let ln: Vec<_> = l.split(' ').collect();
            Instr{
                op: ln[0].parse::<Op>().unwrap(),
                arg: ln[1].parse::<i32>().unwrap()
            }
        }).collect();
    let mut state = State{a:0, pc:0};
    exec(&prog, &mut state);
    println!("{}", state.a);
    for (i,a) in prog.iter().enumerate() {
        let mut patched = prog.clone();
        match a.op {
            Op::Nop => patched[i].op = Op::Jmp,
            Op::Jmp => patched[i].op = Op::Nop,
            _ => continue
        }
        let mut state = State{a:0, pc:0};
        match exec(&patched, &mut state) {
            TermState::Bounds => {
                println!("{}", state.a);
                break;
            },
            TermState::Loop => ()
        }
    }
}
