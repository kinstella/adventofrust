use std::fs;
const FILENAME: &str = "./resources/day03/input.txt";

fn get_char_score(c: char) -> u32 {
    let ascii_val = c as u32;
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
    println!("Part 1 {:?}", vals.iter().sum::<u32>());
}

fn group_badge_score(group: &[String]) -> u32 {
    // TODO: handle if array is too small
    let elves = group.to_vec();
    let elfchars = group[0].chars();
    for e in elfchars {
        let mut has_e = true;
        for elf in elves.iter() {
            if !elf.contains(e) {
                has_e = false;
            }
        }
        if has_e {
            return get_char_score(e);
        }
    }
    return 0; // TODO: if we get here, we've failed to find a match...should throw something instead?
}

pub fn part2() {
    let raw_data: String = read_data(FILENAME);
    let lines: Vec<String> = raw_data.split('\n').map(|s| s.to_string()).collect();
    let mut score = 0;
    for chunk in lines.chunks(3).into_iter() {
        score = score + group_badge_score(chunk);
    }
    println!("Part 2: {}", score);
}
