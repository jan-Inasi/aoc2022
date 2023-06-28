pub fn solve(input: String, is_part_one: bool) -> i32 {
    let (stacks_repr, moves) = match split_input(&input) {
        Some((stacks, moves)) => (stacks, moves),
        _ => {
            println!("WARNING couldn't find stacks or moves in the input");
            return 0;
        }
    };

    let stacks = match stacks_repr.parse::<Stacks>() {
        Ok(stacks) => stacks,
        Err(_) => {
            println!("WARNING couldn't parse stacks' data");
            return 0;
        }
    };

    if is_part_one {
        solve_part_one(stacks, moves)
    } else {
        solve_part_two(stacks, moves)
    }
}

fn solve_part_one(mut stacks: Stacks, moves: &str) -> i32 {
    for (count, from, to) in parse_moves(moves) {
        for _ in 0..count {
            if let Some(item) = stacks.pop_from(from) {
                stacks.push_on(to, item);
            } else {
                println!("WARNING trying to pop from empty stack");
            }
        }
    }

    let top_crates: String = stacks
        .stacks_slice()
        .iter()
        .flat_map(|x| x.last())
        .collect();

    println!("real output: {top_crates}");

    0
}

fn solve_part_two(mut stacks: Stacks, moves: &str) -> i32 {
    let mut counter_stack = Vec::new();
    for (count, from, to) in parse_moves(moves) {
        for _ in 0..count {
            if let Some(item) = stacks.pop_from(from) {
                counter_stack.push(item);
            } else {
                println!("WARNING trying to pop from empty stack");
            }
        }
        for _ in 0..count {
            if let Some(item) = counter_stack.pop() {
                stacks.push_on(to, item);
            } else {
                println!(
                    "WARNING this should never happen; otherwise I have no idea what I'm doing"
                );
            }
        }
    }

    let top_crates: String = stacks
        .stacks_slice()
        .iter()
        .flat_map(|x| x.last())
        .collect();

    println!("real output: {top_crates}");

    0
}

fn split_input(input: &str) -> Option<(&str, &str)> {
    let mut input_splitter = input.split("\n\n");

    let input_stacks = input_splitter.next();
    let input_rearangements = input_splitter.next();
    match (input_stacks, input_rearangements) {
        (Some(stacks), Some(rearangements)) => Some((stacks, rearangements)),
        _ => None,
    }
}

fn parse_num<'a>(num_iter: &mut impl Iterator<Item = &'a str>) -> Option<usize> {
    if let Some(text) = num_iter.next() {
        if let Ok(number) = text.parse::<usize>() {
            return Some(number);
        }
    }
    None
}

fn parse_moves(moves_repr: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    moves_repr.lines().flat_map(|line| {
        let mut num_iter = line.split_whitespace().skip(1).step_by(2);
        let a = parse_num(&mut num_iter);
        let b = parse_num(&mut num_iter);
        let c = parse_num(&mut num_iter);
        if let (Some(a), Some(b), Some(c)) = (a, b, c) {
            Some((a, b, c))
        } else {
            println!("WARNING not every move number could be parsed");
            None
        }
    })
}

struct Stacks {
    pub stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new(nr_of_stacks: usize) -> Self {
        Stacks {
            stacks: vec![Vec::new(); nr_of_stacks],
        }
    }

    pub fn stacks_slice(&self) -> &[Vec<char>] {
        &self.stacks
    }

    pub fn get_stack_mut(&mut self, stack_id: usize) -> &mut Vec<char> {
        &mut self.stacks[stack_id - 1]
    }

    pub fn push_on(&mut self, stack_id: usize, value: char) {
        self.get_stack_mut(stack_id).push(value);
    }

    pub fn pop_from(&mut self, stack_id: usize) -> Option<char> {
        self.get_stack_mut(stack_id).pop()
    }
}

impl std::str::FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines().rev();

        let stack_count = if let Some(number_line) = line_iter.next() {
            (number_line.len() + 1) / 4
        } else {
            return Err(());
        };

        let mut stacks = Stacks::new(stack_count);
        for line in line_iter {
            for (i, symbol) in line.chars().skip(1).step_by(4).enumerate() {
                if i >= stack_count {
                    println!("WARNING more stacks than expected while parsing stacks");
                    break;
                } else if symbol.is_alphabetic() {
                    stacks.push_on(i + 1, symbol);
                }
            }
        }

        Ok(stacks)
    }
}
