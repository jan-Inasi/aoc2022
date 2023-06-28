use aoc2022::*;
use argparse::ArgumentParser;
use regex::Regex;
use std::{fmt::Display, io::Write, path::Path};

const INPUT_DIR: &str = "inputs";

fn main() {
    let (day, do_part_two, init_day) = parse_arguments();

    if init_day {
        init_new_day(day);
        return;
    }

    let fun = match day {
        1 => |x, y| day01::solve(x, y),
        2 => |x, y| day02::solve(x, y),
        3 => |x, y| day03::solve(x, y),
        4 => |x, y| day04::solve(x, y),
        5 => |x, y| day05::solve(x, y),
        _ => {
            println!("the day has not been created yet, sorry …");
            return;
        }
    };

    let file_path = deduce_input_file_path(day);
    if let Result::Ok(text) = std::fs::read_to_string(&file_path) {
        let part_nr = if do_part_two { 2 } else { 1 };
        println!("running part {part_nr} of day {day} problem");
        println!("input from: '{file_path}'");
        fun(text, !do_part_two);
    } else {
        println!("can't find input file at: '{file_path}'");
    }
}

fn deduce_input_file_path(day: i32) -> String {
    format!("{INPUT_DIR}/day{day:0>2}.txt")
}

fn init_new_day(day: i32) {
    let input_dir_exists = Path::new(INPUT_DIR).is_dir();
    let input_file_path = deduce_input_file_path(day);
    let input_file_exists = Path::new(&input_file_path).is_file();

    let day_src_dir = format!("src/day{day:0>2}");
    let day_src_file = format!("{day_src_dir}/mod.rs");
    let src_dir_exists = Path::new(&day_src_dir).is_dir();
    let src_file_exists = Path::new(&day_src_file).is_file();

    if !input_dir_exists {
        if let Err(e) = std::fs::create_dir(&INPUT_DIR) {
            println!("WARNING failed to create input directory");
            println!("{e}");
        } else {
            println!("created input directory at: 'HOME/{INPUT_DIR}'");
        }
    }

    if !input_file_exists {
        if let Err(e) = std::fs::File::create(&input_file_path) {
            println!("WARNING failed to create input file");
            println!("{e}");
        } else {
            println!("created input file at: 'HOME/{input_file_path}'")
        }
    }

    if !src_dir_exists {
        if let Err(e) = std::fs::create_dir(&day_src_dir) {
            println!("WARNING failed to create src directory");
            println!("{e}");
        } else {
            println!("created src directory at: 'HOME/{day_src_dir}'");
        }
    }

    if !src_file_exists {
        match std::fs::File::create(&day_src_file) {
            Err(e) => {
                println!("WARNING failed to create src file");
                println!("{e}");
            }
            Ok(mut file) => {
                println!("created src file at: 'HOME/{day_src_file}'");
                if let Err(_) = file.write_all(SRC_TEMPLATE) {
                    println!("WARNING failed to write day snippet");
                }
            }
        }
    }

    let this_file_text = include_str!("bin.rs");

    let re_line = Regex::new(r"^ *\d+ => \|x, y\| day\d\d::solve\(x, y\),").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();

    let mut idx_to_insert = None;
    let mut found = false;
    for (i, line) in this_file_text.lines().enumerate() {
        if re_line.is_match(line) {
            found = true;
            if let Some(mat) = re_num.find(line) {
                if let Ok(day_nr) = &line[mat.start()..mat.end()].parse::<i32>() {
                    if *day_nr == day {
                        break;
                    } else if *day_nr > day {
                        idx_to_insert = Some(i);
                        break;
                    }
                }
            }
        } else if found {
            idx_to_insert = Some(i);
            break;
        }
    }

    let insert_line_nr = if let Some(x) = idx_to_insert {
        x
    } else {
        println!("WARNING couldn't find the place to insert new function call");
        return;
    };

    for line in this_file_text.lines().take(insert_line_nr) {
        println!("{line}");
    }

    println!("        {day} => |x, y| day{day:0>2}::solve(x, y),");

    for line in this_file_text.lines().skip(insert_line_nr) {
        println!("{line}");
    }
}

fn parse_arguments() -> (i32, bool, bool) {
    let mut day = 1;
    let mut do_part_two = false;
    let mut init_day = false;
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
        ap.refer(&mut init_day).add_option(
            &["--init", "--init_day"],
            argparse::StoreTrue,
            "if specified generating boilerplate code and files for writing a new day",
        );
        ap.parse_args_or_exit();
    }

    if day < 1 || day > 25 {
        println!("There is no day number {day}, you were living a lie!");
        std::process::exit(0);
    }

    (day, do_part_two, init_day)
}

const SRC_TEMPLATE: &[u8] = b"pub fn solve(input: String, is_part_one: bool) {
    let result = if is_part_one {
        solve_part_one(input)
    } else {
        solve_part_two(input)
    };

    println!(\"output: {result}\");
}

fn solve_part_one(input: String) -> i32 {
    0
}

fn solve_part_two(input: String) -> i32 {
    0
}
";
