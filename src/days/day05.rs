use std::{fs, str};
const FILENAME: &str = "./resources/day05/example.txt";

fn read_data(fpath: &str) -> String {
    fs::read_to_string(fpath).expect("Could not read input file")
}

pub fn unpack_crate(c: &str) -> String {
    c.trim().replace("[", "").replace("]", "").to_owned()
}

pub fn build_stacks(initial_stack: &str) -> Vec<Vec<String>> {
    let stacklines = initial_stack.split("\n").collect::<Vec<&str>>();
    let (slacknums, stackdefs) = stacklines.split_last().unwrap();
    // get the last digit for total stack nums from the initial data
    let stacksize = slacknums
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut stacks: Vec<Vec<String>> = vec![vec!["".to_string(); 0]; stacksize as usize];
    for s in stackdefs.iter().map(|i| *i).collect::<Vec<&str>>() {
        let segs = s.as_bytes().chunks(4);
        for (i, sg) in segs.enumerate() {
            let ltrcrate = str::from_utf8(&sg).unwrap();
            let opened = unpack_crate(ltrcrate);
            if opened.len() > 0 {
                let mut newq: Vec<String> = stacks[i].to_vec();
                newq.reverse();
                newq.push(opened);
                newq.reverse();
                stacks[i] = newq;
            }
        }
    }
    stacks.to_owned()
}

pub fn process_ops(mut stacks: Vec<Vec<String>>, op: &str) -> Vec<Vec<String>> {
    let op_nums: Vec<u32> = op
        .replace("move", "")
        .replace("from", "")
        .replace("to", "")
        .trim()
        .split_whitespace()
        .map(|el| el.parse::<u32>().unwrap())
        .collect();

    if op_nums.len() > 1 {
        let (opct, opfrom, opto) = (op_nums[0], op_nums[1] - 1, op_nums[2] - 1);
        for i in 0..opct  {
            print_stacks(&stacks);
            let p = stacks[opfrom as usize].pop().unwrap();
            stacks[opto as usize].push(p);
        }
    }
    stacks
}

pub fn print_stacks( stacks: &Vec<Vec<String>>) {
    for i in stacks {
        println!("{:?}", i);
    }
}

pub fn part1() {
    let raw_data = read_data(FILENAME);
    let input_sections: Vec<&str> = raw_data.split("\n\n").collect::<Vec<&str>>();
    let stack_def = input_sections[0];
    let ops_list = input_sections[1].split("\n").collect::<Vec<&str>>();
    let mut stacks = build_stacks(stack_def);
    for op in ops_list {
        stacks = process_ops(stacks, op);
    }
    for s in stacks {
        print!("{:?}", s.last().unwrap());
    }
}
