use std::{collections::HashMap, fs};
const FNAME: &str = "./resources/day02/data.txt";

const KNOWN_HANDS: [(&str, &str, i32); 9] = [
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

fn shape_for_outcome(p1: &str, outcome: &str) -> String {
    let mut shape: &str = "";
    let outcome_score = match outcome {
        "X" => 0,
        "Y" => 3,
        _ => 6,
    };
    for h in KNOWN_HANDS.iter() {
        if h.0 == p1 && h.2 == outcome_score {
            shape = h.1;
        }
    }
    shape.to_owned()
}

fn hand_score(p1: &str, p2: &str) -> i32 {
    let mut result = 0;

    for h in KNOWN_HANDS.iter() {
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

pub fn day02() {
    let mut scores_pt1: Vec<i32> = Vec::new();
    let mut scores_pt2: Vec<i32> = Vec::new();
    let raw_data = read_data(FNAME);
    let lines = raw_data.split('\n');
    for ln in lines {
        let els: Vec<&str> = ln.split(" ").collect();
        if els.len() == 2 {
            let p1 = els.iter().nth(0).unwrap();
            let p2 = els.iter().nth(1).unwrap();
            scores_pt1.push(calc_score(p1, &p2));
            // for part 2, figure out the shape, then the score
            let shape_used: &str = &shape_for_outcome(p1, p2)[..];
            scores_pt2.push(calc_score(p1, shape_used));
        }
    }
    let score_sum_pt1: i32 = scores_pt1.iter().sum();
    let score_sum_pt2: i32 = scores_pt2.iter().sum();
    println!("Part 1 result: {}", score_sum_pt1);
    println!("Part 2 result: {}", score_sum_pt2);
}
