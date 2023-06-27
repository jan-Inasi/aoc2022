fn main() {
    println!("Hello, world!");

    let contents = std::fs::read_to_string("inputs/day02.txt");

    if let Result::Ok(text) = contents {
        println!("{}", aoc2022::day02::solve(text, false));
    } else {
        println!("file not found");
    }
}
