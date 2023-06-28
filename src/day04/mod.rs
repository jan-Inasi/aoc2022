pub fn solve(input: String, is_part_one: bool) -> i32 {
    if is_part_one {
        solve_part_one(input)
    } else {
        solve_part_two(input)
    }
}

fn solve_part_one(input: String) -> i32 {
    let mut total_count = 0;
    let mut pair_count = 0;
    for ((l1, u1), (l2, u2)) in parse_input(&input) {
        pair_count += 1;
        if l1 <= l2 && u1 >= u2 {
            total_count += 1;
        } else if l2 <= l1 && u2 >= u1 {
            total_count += 1;
        }
    }
    println!("pair count: {pair_count}");
    total_count
}

fn solve_part_two(input: String) -> i32 {
    let mut total_count = 0;
    let mut pair_count = 0;
    for ((l1, u1), (l2, u2)) in parse_input(&input) {
        pair_count += 1;
        if !(u1 < l2 || u2 < l1) {
            total_count += 1;
        }
    }
    println!("pair count: {pair_count}");
    total_count
}

fn parse_section(text: &str) -> Option<(i32, i32)> {
    let mut split_itr = text.split('-');
    let one = split_itr.next();
    let two = split_itr.next();
    let (left, right) = match (one, two) {
        (Some(left), Some(right)) => (left, right),
        _ => return None,
    };

    let lower_bound: Result<i32, _> = left.parse();
    let upper_bound: Result<i32, _> = right.parse();

    match (lower_bound, upper_bound) {
        (Ok(lower_bound), Ok(upper_bound)) => Some((lower_bound, upper_bound)),
        _ => None,
    }
}

fn split_sections(text: &str) -> Option<(&str, &str)> {
    let mut itr = text.split(',');
    let left = itr.next();
    let right = itr.next();
    match (left, right) {
        (Some(left), Some(right)) => Some((left, right)),
        _ => None,
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + '_ {
    let parse_both_sections = |(left, right)| match (parse_section(left), parse_section(right)) {
        (Some(left), Some(right)) => Some((left, right)),
        _ => None,
    };

    input
        .lines()
        .flat_map(split_sections)
        .flat_map(parse_both_sections)
}
