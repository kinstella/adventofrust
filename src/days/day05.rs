use std::{fs, num};
const FILENAME: &str = "./resources/day05/example.txt";

fn read_data(fpath: &str) -> String {
    fs::read_to_string(fpath).expect("Could not read input file")
}

pub fn build_stacks(initial_stack: &str) -> Vec<Vec<String>> {
    let stacklines = initial_stack.split("\n").collect::<Vec<&str>>();
    let (slacknums, stackdefs) = stacklines.split_last().unwrap();
    let mut stacksize = slacknums
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    // get the last digit for total stack nums from the initial data
    println!("num stacks: {:?}", stacksize);
    for s in stackdefs.iter() {
        println!("rem stackline: {}", s);
    }
    let mut stacks: Vec<Vec<String>> = vec![vec!["".to_string(); 0]; stacksize as usize];

    stacks.to_owned()
}

pub fn part1() {
    let raw_data = read_data(FILENAME);
    let input_sections: Vec<&str> = raw_data.split("\n\n").collect::<Vec<&str>>();
    let stack_def = input_sections[0];
    let moves_list = input_sections[1];
    let stacks = build_stacks(stack_def);

    println!("stacks:\n\n{:?}", stacks);
    println!("\n\nMoves list:\n{}", moves_list);
}
