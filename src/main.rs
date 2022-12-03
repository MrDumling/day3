use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_input(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn get_rucksacks(input: Lines<BufReader<File>>) -> Vec<(String, String)> {
    input.fold(Vec::new(), |mut acc, current_line| {
        let current_line = current_line.unwrap();
        let line_length = current_line.len();

        let left = current_line[0..line_length/2].to_string();
        let right = current_line[line_length/2..].to_string();

        acc.push((left, right));
        acc
    })
}

fn group_lines<const N: usize>(input: Lines<BufReader<File>>) -> Vec<[String; N]> {
    let mut result = Vec::new();
    let mut current_chunk = [(); N].map(|_| String::new());

    for (index, current_line) in input.enumerate() {
        let current_line = current_line.unwrap();
        current_chunk[index % N] = current_line;

        if index % N == N - 1 {
            result.push(current_chunk);
            current_chunk = [(); N].map(|_| String::new());
        }
    }
    
    result
}

fn value_char(c: char) -> u64 {
    if c.is_lowercase() {
        c as u64 - 96
    } else {
        c as u64 - 64 + 26
    }
}

fn get_shared_character<const N: usize>(compared_strings: [String; N]) -> char {
    use std::collections::HashSet;
    let mut result: HashSet<char> = compared_strings[0].chars().into_iter().collect();

    for current_string in &compared_strings[1..] {
        let retained_chars: Vec<char> = current_string.chars().into_iter().collect();
        result.retain(|x| retained_chars.contains(x));
    }

    if result.len() != 1 {
        panic!("Only one character should be shared, instead found {:?}", result);
    }

    result.into_iter().next().unwrap()
}

fn puzzle_1() {
    let mut total_value = 0;
    let rucksacks = get_rucksacks(get_input("./input.txt"));

    for current_rucksack in rucksacks {
        let shared_letter = get_shared_character([current_rucksack.0, current_rucksack.1]);
        total_value += value_char(shared_letter);
    }

    println!("{:?}", total_value);
}

fn puzzle_2() {
    let mut total_value = 0;

    let groups: Vec<[String; 3]> = group_lines(get_input("./input.txt"));
    for current_group in groups {
        let shared_badge = get_shared_character(current_group);
        total_value += value_char(shared_badge);
    }

    println!("{:?}", total_value);
}

fn main() {
    puzzle_1();
    puzzle_2();
}