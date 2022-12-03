use std::fs;

const FILENAME: &str = "./resources/day03/input.txt";

fn get_char_score(c: char) -> u32 {
    let ascii_val = c as u32;
    println!("ascii: {}", ascii_val);
    if ascii_val >= 65 && ascii_val <= 91 {
        return ascii_val - 38;
    }
    return ascii_val - 96;
}

fn find_priority(line: &str) -> char {
    let linelen = line.len();
    let (lstr, rstr) = line.split_at(linelen / 2);
    let rchars: Vec<char> = rstr.trim().chars().collect();
    for leftc in lstr.chars() {
        if rchars.contains(&leftc) {
            println!("both contain: {}", leftc);
            return leftc;
        }
    }
    return '\0';
}

fn read_data(fpath: &str) -> String {
    fs::read_to_string(fpath).expect("Could not read input file")
}

pub fn part1() {
    let raw_data = read_data(FILENAME);
    let lines = raw_data.split('\n');
    let mut vals: Vec<u32> = Vec::new();
    for ln in lines {
        if ln.len() > 0 {
            vals.push(get_char_score(find_priority(ln)));
        }
    }
    println!("{:?}", vals.iter().sum::<u32>());
}
