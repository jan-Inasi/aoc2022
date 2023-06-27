use std::convert::TryFrom;

pub fn solve(input: String, part_one: bool) -> i32 {
    if part_one {
        solve_part_one(input)
    } else {
        solve_part_two(input)
    }
}

fn solve_part_one(input: String) -> i32 {
    let mut total_score = 0;
    for (left, right) in input_parser(&input) {
        if let (Ok(oponent_play), Ok(my_play)) = (RPS::try_from(left), RPS::try_from(right)) {
            total_score += score(oponent_play, my_play);
        }
    }
    total_score
}

fn solve_part_two(input: String) -> i32 {
    let mut total_score = 0;
    for (left, right) in input_parser(&input) {
        if let Ok(oponent_play) = RPS::try_from(left) {
            total_score += score_part_two(oponent_play, right);
        }
    }

    total_score
}

fn input_parser(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
    input.lines().filter(|x| x.len() == 3).flat_map(|line| {
        let pair: Vec<_> = line.split(" ").collect();
        if pair.len() != 2 || pair[0].len() != 1 || pair[1].len() != 1 {
            return None;
        }

        if let Some(left) = pair[0].chars().next() {
            if let Some(right) = pair[1].chars().next() {
                return Some((left, right));
            }
        }

        None
    })
}

#[derive(PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    pub fn beats(&self, other: RPS) -> bool {
        if *self == RPS::Rock && other == RPS::Scissors {
            return true;
        } else if *self == RPS::Paper && other == RPS::Rock {
            return true;
        } else if *self == RPS::Scissors && other == RPS::Paper {
            return true;
        }
        false
    }

    pub fn loses_with(self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    pub fn draws_with(self) -> RPS {
        self
    }

    pub fn wins_with(self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

impl std::convert::TryFrom<char> for RPS {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(RPS::Rock),
            'B' | 'Y' => Ok(RPS::Paper),
            'C' | 'Z' => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

fn score(oponent_play: RPS, your_play: RPS) -> i32 {
    let points = if oponent_play == your_play {
        3
    } else if your_play.beats(oponent_play) {
        6
    } else {
        0
    };

    points + your_play as i32
}

fn score_part_two(oponent_play: RPS, how_to_play: char) -> i32 {
    let (lose, draw, win) = ('X', 'Y', 'Z');
    if how_to_play == lose {
        0 + oponent_play.wins_with() as i32
    } else if how_to_play == draw {
        3 + oponent_play.draws_with() as i32
    } else if how_to_play == win {
        6 + oponent_play.loses_with() as i32
    } else {
        println!("WARNING unknown character: {how_to_play}");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let input = "A Y\nB X\nC Z\n";

        assert_eq!(solve(input.into(), true), 15);
    }

    #[test]
    fn test_example_part_two() {
        let input = "A Y\nB X\nC Z\n";

        assert_eq!(solve(input.into(), false), 12);
    }
}
