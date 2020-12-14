use std::io::{stdin, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Loc {Empty, Occupied, Floor}

fn count_adjacent(input: &[Vec<Loc>], x: usize, y:usize) -> u32 {
    let offsets: [(i32,i32);8] = [(1,-1), (1,0), (1,1),
                                  (0,-1),        (0,1),
                                  (-1,-1), (-1,0), (-1,1)];
    offsets.iter()
        .map(|(yo,xo)| if let Some(l) = input.get((y as i32 + *yo) as usize) {
            if let Some(e) = l.get((x as i32 + *xo) as usize) {
                match *e {
                    Loc::Occupied => 1,
                    _ => 0
                }
            } else {0}
        } else {0}).sum()
}

fn next_p1(input: &[Vec<Loc>]) -> Vec<Vec<Loc>> {
    input.iter().enumerate()
        .map(|(y, l)| l.iter().enumerate()
             .map(|(x, e)|
                  if *e == Loc::Floor {*e} else{ 
                      match count_adjacent(input, x, y){
                          0 => Loc::Occupied,
                          4..=8 => Loc::Empty,
                          _ => *e
                      }
                  }).collect()).collect()
}

fn traceline(input: &[Vec<Loc>], y:usize, x:usize, dy: i32, dx: i32)
             -> Option<Loc> {
    let mut cx = x as i32 + dx;
    let mut cy = y as i32 + dy;
    loop {
        let e = input.get(cy as usize)?.get(cx as usize)?;
        match e {
            Loc::Floor => (),
            _ => return Some(*e)
        }
        cx += dx;
        cy += dy;
    }
}

fn count_scan(input: &[Vec<Loc>], x: usize, y: usize) -> usize {
    let offsets: [(i32,i32);8] = [(1,-1), (1,0), (1,1),
                                  (0,-1),        (0,1),
                                  (-1,-1), (-1,0), (-1,1)];
    offsets.iter()
        .map(|(yo, xo)| match traceline(input, y, x, *yo, *xo) {
            Some(Loc::Occupied) => 1,
            _ => 0
        }).sum()
}

fn next_p2(input: &[Vec<Loc>]) -> Vec<Vec<Loc>> {
    input.iter().enumerate()
        .map(|(y, l)| l.iter().enumerate()
             .map(|(x, e)| if *e == Loc::Floor {*e} else {
                 match count_scan(input, x, y) {
                     0 => Loc::Occupied,
                     5..=8 => Loc::Empty,
                     _ => *e
                 }
             }).collect()).collect()
}

fn main() {
    let input = stdin().lock().lines()
        .map(|l| l.unwrap().chars()
             .map(|c| match c {
                 '.' => Loc::Floor,
                 'L' => Loc::Empty,
                 '#' => Loc::Occupied,
                 _ => panic!("invalid input {}", c)
             }).collect::<Vec<Loc>>())
        .collect::<Vec<Vec<Loc>>>();
    let mut curr = input.clone();
    loop {
        let nx = next_p1(&curr);
        if nx == curr {break;}
        curr = nx;
    }
    println!("{}",
             curr.iter()
             .map(|l| l.iter()
                  .map(|e| match e {Loc::Occupied => 1, _ => 0})
                  .sum::<usize>())
             .sum::<usize>());
    curr = input.clone();
    loop {
        let nx = next_p2(&curr);
        if nx == curr {break;}
        curr = nx;
    }
    println!("{}",
             curr.iter()
             .map(|l| l.iter()
                  .map(|e| match e {Loc::Occupied => 1, _ => 0})
                  .sum::<usize>())
             .sum::<usize>());    
}
