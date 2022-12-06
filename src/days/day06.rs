use std::{fs, str};
use std::collections::HashSet;
const FILENAME: &str = "./resources/day06/input.txt";

fn read_data(fpath: &str) -> String {
    fs::read_to_string(fpath).expect("Could not read input file")
}

pub fn find_unique_set(ln : usize, orig_str : String) {
    let mut vstr = orig_str.chars().map(|c| c.to_string()).collect::<Vec<String>>().to_owned();
    let mut charct = 0;
    let mut items = vstr.iter().take(ln).map(|c| c.to_string()).collect::<HashSet<String>>().clone();
    while items.len() < ln {
        vstr = vstr[1..].iter().map(|c| c.to_string()).collect();
        items = vstr.iter().take(ln).map(|c| c.to_string()).collect::<HashSet<String>>().clone();
        charct = 1 + charct;
    }
    println!("At char {}, we found {:?}", charct + ln, items);
}

pub fn part1() {
    let raw_data = read_data(FILENAME);
    let marker_size = 4;
    find_unique_set(marker_size, raw_data);
}

pub fn part2() {
    let raw_data = read_data(FILENAME);
    let marker_size = 14;
    find_unique_set(marker_size, raw_data);
}