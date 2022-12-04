use std::fs;
const FILENAME: &str = "./resources/day04/input.txt";

fn read_data(fpath: &str) -> String {
    fs::read_to_string(fpath).expect("Could not read input file")
}

pub fn pair_to_minmax(pr: &str) -> (u32, u32) {
    let nms: Vec<u32> = pr
        .split('-')
        .map(|n| n.to_string().parse::<u32>().expect("could not parse pair"))
        .collect();
    (nms[0], nms[1])
}

pub fn has_contained_pair(ln: &str) -> bool {
    let sides: Vec<&str> = ln.split(',').collect::<Vec<&str>>();
    let (lmin, lmax) = pair_to_minmax(sides[0]);
    let (rmin, rmax) = pair_to_minmax(sides[1]);
    ((lmin <= rmin) && (lmax >= rmax)) || ((lmin >= rmin) && (lmax <= rmax))
}

pub fn has_overlap(ln: &str) -> bool {
    let sides: Vec<&str> = ln.split(',').collect::<Vec<&str>>();
    let (lmin, lmax) = pair_to_minmax(sides[0]);
    let (rmin, rmax) = pair_to_minmax(sides[1]);
    let mut overlaps: bool = true;
    if (lmin < rmin) && (lmax < rmin) {
        overlaps = false;
    }
    if (rmin < lmin) && (rmax < lmin) {
        overlaps = false;
    }
    overlaps
}

pub fn part1() {
    let raw_data = read_data(FILENAME);
    let lines = raw_data.split('\n');
    let mut contain_count = 0;
    for ln in lines {
        if ln.len() > 0 && has_contained_pair(ln) {
            contain_count = contain_count + 1;
        }
    }
    println!("Part 1: {:?}", contain_count);
}

pub fn part2() {
    let raw_data = read_data(FILENAME);
    let lines = raw_data.split('\n');
    let mut overlap_count = 0;
    for ln in lines {
        if ln.len() > 0 && has_overlap(ln) {
            overlap_count = overlap_count + 1;
        }
    }
    println!("Part 2: {:?}", overlap_count);
}
