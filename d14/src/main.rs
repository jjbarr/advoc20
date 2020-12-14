use std::{fmt, str};
use std::io::{stdin, BufRead};
use regex::Regex;
use im::HashMap;

#[derive(Debug)]
struct ParseMaskError {
    failing: char
}

impl fmt::Display for ParseMaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unrecognized mask char '{}'", self.failing)
    }
}
impl std::error::Error for ParseMaskError {}

#[derive(Clone, Copy, PartialEq)]
struct Mask {
    am: u64,
    om: u64
}

impl Mask {
    fn new() -> Self {
        Mask{om: 0, am: u64::MAX}
    }
    fn apply(&self, x: u64) -> u64 {
        (x | self.om) & self.am
    }
    fn apply_address(&self, a: u64, xbvals: u64) -> u64 {
        let mut xbts = self.xbts();
        let mut xbm = Self::new();
        let mut xbtloc = 0;
        let mut xbvloc = 0;
        while xbts != 0 {
            if xbts & 1 == 1 {
                match xbvals & (1 << xbvloc) {
                    0 => xbm.am &= !(1 << xbtloc),
                    _ => xbm.om |= 1 << xbtloc,
                }
                xbvloc += 1;
            }
            xbts >>= 1;
            xbtloc += 1;
        }
        xbm.apply(a | self.om)
    }
    fn xbts(&self) -> u64 {
        (!self.om & self.am) & 0xFFFFFFFFF
    }
}

#[test]
fn test_mask() {
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse::<Mask>().unwrap();
    assert_eq!(mask.om, 0b1000000);
    assert_eq!(mask.am, u64::MAX - 0b10);
    assert_eq!(mask.apply(11), 73);
}

impl str::FromStr for Mask {
    type Err = ParseMaskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Mask::new();
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                'X' => (),
                '1' => m.om |= 1 << i,
                '0' => m.am &= !(1 << i),
                _ => Err(Self::Err{failing: c})?
            }
        }
        Ok(m)
    }
}

struct Machine {
    mask: Mask,
    mem: HashMap<u64, u64>
}

enum Ops {SetMask(Mask), AssignMem(u64, u64)}

fn count_bits(mut n: u64) -> u64 {
    let mut b = 0;
    while n != 0 {
        b += if n & 1 == 1 {1} else {0};
        n >>= 1;
    }
    b
}

impl Ops {
    fn execv1(&self, m: &Machine) -> Machine {
        match self {
            Self::SetMask(mask) => Machine{mask: *mask, mem: m.mem.clone()},
            Self::AssignMem(a,v) =>
                Machine{mask: m.mask, mem: m.mem.update(*a, m.mask.apply(*v))}
        }
    }
    fn execv2(&self, m: &Machine) -> Machine {
        match self {
            Self::SetMask(mask) => Machine{mask: *mask, mem: m.mem.clone()},
            Self::AssignMem(a, v) =>
                Machine{
                    mask: m.mask,
                    mem: (0x0u64..2u64.pow(count_bits(m.mask.xbts()) as u32))
                        .fold(m.mem.clone(), |mem, xbvals|
                              mem.update(m.mask.apply_address(*a, xbvals), *v))
                }
        }
    }
}

fn main() {
    let mask_re = Regex::new(r"^mask = ([01X]+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();
    let ops = stdin().lock().lines()
        .map(|l| {
            let l = l.unwrap();
            if let Some(caps) = mask_re.captures(&l) {
                Ops::SetMask(caps[1].parse::<Mask>().unwrap())
            } else if let Some(caps) = mem_re.captures(&l) {
                Ops::AssignMem(caps[1].parse::<u64>().unwrap(),
                               caps[2].parse::<u64>().unwrap())
            } else {
                panic!("unrecognized line!");
            }
        }).collect::<Vec<_>>();
    let finmac = ops.iter().
        fold(Machine{mask: Mask::new(), mem: HashMap::new()},
             |m, o| o.execv1(&m));
    println!("{}", finmac.mem.values().sum::<u64>());
    let finmac2 = ops.iter().fold(
        Machine{mask: Mask::new(), mem: HashMap::new()}, |m, o| o.execv2(&m));
    println!("{}", finmac2.mem.values().sum::<u64>());
}
