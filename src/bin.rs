use aoc2022::day01;

fn main() {
    println!("Hello, world!");

    let contents = std::fs::read_to_string("inputs/day01.txt");

    if let Result::Ok(text) = contents {
        println!("{}", day01::solve(text, 3));
    } else {
        println!("file not found");
    }
}
