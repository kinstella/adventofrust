use std::{collections::HashMap, fs, os::raw};
const FNAME: &str = "./resources/day02/data.txt";

fn hand_score(p1: &str, p2: &str) -> i32 {
    let mut result = 0;
    let known_hands = vec![
        ("A", "X", 3),
        ("A", "Y", 6),
        ("A", "Z", 0),
        ("B", "X", 0),
        ("B", "Y", 3),
        ("B", "Z", 6),
        ("C", "X", 6),
        ("C", "Y", 0),
        ("C", "Z", 3),
    ];
    for h in known_hands.iter() {
        if h.0 == p1 && h.1 == p2 {
            result = h.2;
        }
    }
    result
}

fn calc_score(p1: &str, p2: &str) -> i32 {
    let mut shape_score = HashMap::new();
    shape_score.insert("X", 1);
    shape_score.insert("Y", 2);
    shape_score.insert("Z", 3);

    hand_score(p1, p2) + shape_score.get(p2).unwrap()
}

fn read_data(fpath: &str) -> String {
    fs::read_to_string(fpath).expect("Could not read input file")
}

pub fn day02_part1() {
    let mut scores: Vec<i32> = Vec::new();
    let raw_data = read_data(FNAME);
    println!("rawdata:{}", raw_data);
    let lines = raw_data.split('\n');
    for ln in lines {
        println!("ln:{}", ln);
        let els: Vec<&str> = ln.split(" ").collect();
        if (els.len() == 2) {
            let p1 = els.iter().nth(0).unwrap();
            let p2 = els.iter().nth(1).unwrap();
            scores.push(calc_score(p1, p2));
        }
    }

    let score_sum: i32 = scores.iter().sum();
    println!("result: {}", score_sum);
}
