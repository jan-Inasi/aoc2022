use argparse::ArgumentParser;

fn main() {
    let (day, do_part_two) = parse_arguments();

    let fun = match day {
        1 => |x, y| aoc2022::day01::solve(x, y),
        2 => |x, y| aoc2022::day02::solve(x, y),
        3 => |x, y| aoc2022::day03::solve(x, y),
        4 => |x, y| aoc2022::day04::solve(x, y),
        5 => |x, y| aoc2022::day05::solve(x, y),
        _ => {
            println!("the day has not been created yet, sorry …");
            return;
        }
    };

    let file_path = format!("inputs/day{day:0>2}.txt");
    if let Result::Ok(text) = std::fs::read_to_string(&file_path) {
        let part_nr = if do_part_two { 2 } else { 1 };
        println!("running part {part_nr} of day {day} problem");
        println!("input from: '{file_path}'");
        println!("output: {}", fun(text, !do_part_two));
    } else {
        println!("can't find input file at: '{file_path}'");
    }
}

fn parse_arguments() -> (i32, bool) {
    let mut day = 1;
    let mut do_part_two = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("choose the day of the advent");
        ap.refer(&mut day)
            .add_argument("day", argparse::Store, "the number of the advent day");
        ap.refer(&mut do_part_two).add_option(
            &["-t", "-2", "--part_two"],
            argparse::StoreTrue,
            "if specified running the second part of the day problem",
        );
        ap.parse_args_or_exit();
    }

    if day < 1 || day > 25 {
        println!("There is no day number {day}, you were living a lie!");
        std::process::exit(0);
    }

    (day, do_part_two)
}
