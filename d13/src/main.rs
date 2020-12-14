use std::io::{stdin, BufRead};

enum Bus {
    Bus(i32),
    Skip
}

//stolen from RosettaCode
fn egcd(a:i64, b:i64) -> (i64, i64, i64) {
    if a == 0 {(b,0,1)}
    else {
        let (g, x, y) = egcd(b%a, a);
        (g, y - (b / a) * x, x)
    }
}

//stolen from RosettaCode
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {Some((x%n+n) % n)}
    else {None}
}

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let ets = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let busses = lines.next().unwrap().unwrap().split(',')
        .map(|n|
             if n == "x" {
                 Bus::Skip
             } else {
                 Bus::Bus(n.parse::<i32>().unwrap())
             }).collect::<Vec<Bus>>();
    drop(lines);
    drop(stdin); //why not?
    let buswait = busses.iter()
        .filter_map(|b| match b {
            Bus::Bus(id) => Some(id),
            Bus::Skip => None
        }).map(|id| (id, id - (ets % id)))
        .min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("{}", buswait.0 * buswait.1);
    let idmod = busses.iter().
        filter_map((||{
            let mut c = 0;
            move |b: &Bus| {
                c+=1;
                match *b {
                    Bus::Bus(id) => Some((id as i64,
                                          ((id as i64 - (c-1) + id as i64)
                                           % id as i64))),
                    Bus::Skip => None
                }
            }
        })()).collect::<Vec<(i64, i64)>>();
    let m = idmod.iter().map(|(id, _)| *id).product::<i64>();
    println!("{}", idmod.iter()
             .map(|(id, imod)| imod * (m/id) * mod_inv(m/id, *id).unwrap())
             .sum::<i64>() % m);
    
}
