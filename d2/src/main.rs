use std::io::{stdin, BufRead};
use regex::Regex; 

#[derive(Clone, Copy)]
struct Rule {
    min: usize,
    max: usize,
    c: char
}

struct Pass {
    word : String,
    rule : Rule
}

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let passes = stdin().lock().lines()
        .map(|l| {
            let u = l.unwrap();
            let c = re.captures(&u).unwrap();
            Pass {
                word: String::from(&c[4]),
                rule: Rule {
                    min: c[1].parse::<usize>().unwrap(),
                    max: c[2].parse::<usize>().unwrap(),
                    c: c[3].chars().next().unwrap()
                }
            }
        }).collect::<Vec<Pass>>();
    println!("{}", passes.iter().filter(|p| {
        let c = p.word.matches(p.rule.c).count();
        c as usize >= p.rule.min && c as usize <= p.rule.max
    }).count());
    println!("{}", passes.iter().filter(|p| {
        p.word.chars().nth(p.rule.min - 1)
            .map(|c| p.rule.c == c).unwrap_or(false)
            ^ p.word.chars().nth(p.rule.max - 1)
            .map(|c| p.rule.c == c).unwrap_or(false)
    }).count())
}
