use std::collections::HashMap;

pub fn one() {
    let input  = include_str!("inputs/one_input.txt");
    let (mut first, mut second) = get_lists(input);
    first.sort();
    second.sort();
    let total: i64 = first
        .iter()
        .zip(second)
        .map(|(a, b)| (a-b).abs())
        .sum();
    println!("{total}")
}

pub fn two() {
    let input = include_str!("inputs/one_input.txt");
    let (first, second) = get_lists(input);
    let frequencies = create_frequency_table(&second);
    let mut total: i64 = 0;
    for number in &first {
        if let Some(frequency) = frequencies.get(number) {
            total += frequency *number;
        } 
    }

    println!("{total}")
}

fn get_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut first: Vec<i64> = Vec::new();
    let mut second: Vec<i64> = Vec::new();
    input
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            let first_int: i64 = parts[0].parse().unwrap();
            let second_int: i64 = parts[1].parse().unwrap();
            first.push(first_int);
            second.push(second_int);
        });
    (first, second)
}

fn create_frequency_table(numbers: &[i64]) -> HashMap<i64, i64> {
    let mut frequency_table: HashMap<i64, i64> = HashMap::new();
    for &number in numbers {
        *frequency_table.entry(number).or_insert(0) += 1;
    }
    frequency_table
}