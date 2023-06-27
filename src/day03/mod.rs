use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: String, part_one: bool) -> i32 {
    if part_one {
        solve_part_one(input)
    } else {
        solve_part_two(input)
    }
}

fn solve_part_one(input: String) -> i32 {
    let mut total_sum = 0;
    for line in input.lines().filter(|x| x.len() > 0) {
        let half_ptr = line.len() / 2;
        let first_half: HashSet<char> = (&line[..half_ptr]).chars().collect();
        let second_half: HashSet<char> = (&line[half_ptr..]).chars().collect();

        let in_both = first_half.intersection(&second_half);
        if let Some(item) = in_both.into_iter().next() {
            total_sum += calc_priority(item);
        }
    }

    total_sum as i32
}

fn solve_part_two(input: String) -> i32 {
    let mut total_sum = 0;

    let mut tripleter = input.lines().filter(|x| x.len() > 0).tuples();
    for (one, two, three) in tripleter.by_ref() {
        let mut shared_chars = find_chars_shared_by_strings(&[one, two, three]).into_iter();

        if let Some(item) = shared_chars.next() {
            total_sum += calc_priority(&item);

            if shared_chars.next() != None {
                println!("WARNING found more than one char shared in triplet");
            }
        } else {
            println!("WARNING not found any char shared between triplet");
        }
    }
    if tripleter.into_buffer().len() > 0 {
        println!("WARNING the number of gropus were not divisible by 3");
    }

    total_sum as i32
}

fn find_chars_shared_by_strings(texts: &[&str]) -> HashSet<char> {
    if texts.len() == 0 {
        return HashSet::<char>::new();
    }
    let mut shared_chars: HashSet<char> = texts[0].chars().collect();
    for text in texts.iter().skip(1) {
        let second_set: HashSet<char> = text.chars().collect();
        let intersection = shared_chars.intersection(&second_set);
        shared_chars = intersection.into_iter().map(|x| *x).collect();
    }
    shared_chars
}

fn calc_priority(item: &char) -> u32 {
    if !item.is_ascii_alphabetic() {
        println!("item is not alphabetic");
    }

    let value = *item as u32;

    if item.is_lowercase() {
        value - 'a' as u32 + 1
    } else if item.is_uppercase() {
        value - 'A' as u32 + 27
    } else {
        print!("WARNING this shouldn't be possible to run");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_example_part_one() {
        assert_eq!(solve(INPUT.into(), true), 157);
    }

    #[test]
    fn test_example_part_two() {
        assert_eq!(solve(INPUT.into(), false), 70);
    }
}
