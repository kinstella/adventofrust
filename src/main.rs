use std::fs;

fn sum_elf(e: &str) -> i32 {
    e.split_whitespace()
        .map(|n| {
            n.parse::<i32>()
                .expect("\nError: Could not parse to number.\n")
        })
        .sum()
}

fn read_data(fpath: &str) -> Vec<i32> {
    let contents: String = fs::read_to_string(fpath).expect("Could not read input file");
    contents.split("\n\n").map(|e| sum_elf(e)).collect()
}

fn main() {
    const FNAME: &str = "./resources/day01/data.txt";
    let mut elf_cals = read_data(FNAME);
    elf_cals.sort();
    elf_cals.reverse();
    println!("Part 1: {}", elf_cals.first().unwrap());
    println!("Part 1: {:?}", elf_cals.iter().take(3).sum::<i32>());
}
