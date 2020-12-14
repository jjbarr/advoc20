use std::io::{stdin, BufRead};

#[derive(Clone, PartialEq)]
struct Waypoint {
    x: f64,
    y: f64,
}

impl Waypoint {
    fn new() -> Self {
        Waypoint {x: 10f64, y: 1f64}
    }
    fn rotate(&self, deg: f64) -> Self {
        let theta = deg.to_radians();
        Waypoint {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.y * theta.cos() + self.x * theta.sin()
        }
    }
}

#[derive(Clone, PartialEq)]
struct Ship {
    x: f64,
    y: f64,
    head: f64,
    way: Waypoint
}

impl Ship {
    fn new() -> Self {
        Ship {x: 0f64, y: 0f64, head: 0f64, way: Waypoint::new()}
    }
    fn dist(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
}

enum Action {
    North(f64),
    South(f64),
    East(f64),
    West(f64),
    Left(f64),
    Right(f64),
    Forward(f64)
}

impl Action {
    fn exec_ship(&self, tgt: Ship) -> Ship {
        let mut dest = tgt.clone();
        match self {
            Self::North(d) => dest.y += *d,
            Self::South(d) => dest.y -= *d,
            Self::East(d) => dest.x += *d,
            Self::West(d) => dest.x -= *d,
            Self::Left(d)  => dest.head += *d,
            Self::Right(d) => dest.head -= *d,
            Self::Forward(d) => {
                let theta = dest.head.to_radians();
                dest.x += theta.sin() * *d;
                dest.y += theta.cos() * *d;
            }
        }
        dest
    }

    fn exec_waypoint(&self, tgt: Ship) -> Ship {
        let mut dest = tgt.clone();
        match self {
            Self::North(d) => dest.way.y += *d,
            Self::South(d) => dest.way.y -= *d,
            Self::East(d) => dest.way.x += *d,
            Self::West(d) => dest.way.x -= *d,
            Self::Forward(d) => {
                dest.x += dest.way.x * *d;
                dest.y += dest.way.y * *d;
            }
            Self::Left(d) => dest.way = dest.way.rotate(*d),
            Self::Right(d) => dest.way = dest.way.rotate(-*d)
        }
        dest
    }
}

fn main() {
    let actions: Vec<Action> = stdin().lock().lines()
        .map(|l| {
            let l = l.unwrap();
            let n = l[1..].parse::<f64>().unwrap();
            match l.chars().nth(0).unwrap() {
                'N' => Action::North(n),
                'S' => Action::South(n),
                'E' => Action::East(n),
                'W' => Action::West(n),
                'L' => Action::Left(n),
                'R' => Action::Right(n),
                'F' => Action::Forward(n),
                _ => panic!("Unknown character in input stream!")
            }
        }).collect();
    let endstate = actions.iter().fold(Ship::new(), |s, a| a.exec_ship(s));
    println!("{}", endstate.dist());
    let p2end = actions.iter().fold(Ship::new(), |s, a| a.exec_waypoint(s));
    println!("{}", p2end.dist());
}
