use std::env;

mod one;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <day> <part>");
        return;
    }
    let day: i32 = args[1].parse().expect("Day must be a number");
    let part: i32 = args[2].parse().expect("Part must be a number");

    match (day, part) {
        (1, 1) => one::one(),
        (1, 2) => one::two(),
        _ => println!("Day {} part {} not implemented", day, part),
    }   
}
