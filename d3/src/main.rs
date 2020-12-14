use std::io::{stdin, BufRead};

#[derive(Copy,Clone,Debug)]
enum Object {Tree, None}

fn path_count(path: &Vec<Vec<Object>>, h: usize, v: usize) -> usize {
    path.iter().step_by(v).enumerate()
        .filter(|(i, l)| match l[h*i % l.len()] {
            Object::Tree => true,
            Object::None => false
        }).count()
}

fn main() {
    let path = stdin().lock().lines()
        .map(|l| l.unwrap().chars()
             .map(|c| match c {
                 '.' => Object::None,
                 '#' => Object::Tree,
                 _ => {
                     eprintln!("Unknown character on input: {}", c);
                     std::process::exit(1);
                 }
             }).collect())
        .collect::<Vec<Vec<Object>>>();
    println!("{}", path_count(&path, 3, 1));
    println!("{}", path_count(&path, 1, 1)
             * path_count(&path, 3, 1)
             * path_count(&path, 5, 1)
             * path_count(&path, 7, 1)
             * path_count(&path, 1, 2));
}
