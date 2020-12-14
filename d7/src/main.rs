use std::io::{stdin, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

fn part_2(b: &str, t: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    t.get(b).unwrap().iter().fold(0, |a, (i, b)| a + i + i * part_2(b,t))
}

fn main() {
    let contains_re = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    let contains = stdin().lock().lines()
        .map(|l| {
            let l = l.unwrap();
            let spl = l.split(" bags contain ").collect::<Vec<&str>>();
            let contains = contains_re.captures_iter(spl[1])
                .map(|c| (c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                          String::from(c.get(2).unwrap().as_str())))
                .collect::<Vec<(i32, String)>>();
            (String::from(spl[0]), contains)
        }).collect::<HashMap<_,_>>();
    let contained_in = {
        let mut h : HashMap<String, Vec<String>> = HashMap::new();
        contains.iter()
        .flat_map(|(k,v)| v.iter().map(move |(_,e)| (e.clone(),k)))
            .for_each(|(k, v)|
                      if h.contains_key(&k) {
                          h.get_mut(&k).unwrap().push(v.clone())
                      }
                      else {
                          h.insert(k, vec![v.clone()]);
                      });
        h
    };
    {
        let mut bq = VecDeque::new();
        let mut seen = HashSet::new();
        bq.push_back("shiny gold");
        while let Some(b) = bq.pop_front() {
            seen.insert(b);
            if let Some(v) = contained_in.get(b) {
                v.iter()
                    .for_each(|s|
                              if !seen.contains(s.as_str()) {bq.push_back(s)});
            }
        }
        println!("{}", seen.len() - 1); //shiny gold bags don't count.
    }
    println!("{}", part_2("shiny gold", &contains));
}
