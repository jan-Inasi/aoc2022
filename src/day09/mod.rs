use std::collections::HashSet;
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::Add;

/*
what did I learn:
1. Option.ok_or(Err) zamienia Option na porządany error
2. Error.ok().ok_or(Err) może podmienić error (nie wiem czy jest bardziej idiomatyczny sposób)
3. nie da się iterować po wektorze i akualizować następnego elementu poprzednim
    najpierw, w poprzedniej iteracji, trzeba skopiować wartość poprzedniego do tymczasowej zmiennej (chyba)
4. Vec.last() zwraca Option<&Elem> z referencją do elementu co wszystko knoci gdy wcześniej porzyczyliśmy mutowalnie ten wektor
    aby wziąć kopię ostatniego elementu należy wywołać Vec.last().copied()
*/

pub fn solve(input: String, is_part_one: bool) {
    let result = if is_part_one {
        solve_part_one(input)
    } else {
        solve_part_two(input)
    };

    println!("output: {result}");
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, i32)> + '_ {
    input
        .lines()
        .map(|line| {
            let mut word_iter = line.split(" ");

            let dir_text = word_iter.next().ok_or("no 1st element")?;
            let steps_text = word_iter.next().ok_or("no 2nd element")?;

            let dir: Direction = dir_text.try_into().ok().ok_or("direction parsing error")?;
            let steps: i32 = steps_text.parse().ok().ok_or("steps parsing error")?;

            Ok((dir, steps))
        })
        .flat_map(
            |proxy: Result<(Direction, i32), &'static str>| match proxy {
                Err(text) => {
                    println!("warning! {text}");
                    None
                }
                Ok(good_case) => Some(good_case),
            },
        )
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn vector(&self) -> Point {
        match &self {
            Direction::Down => Point::new(0, -1),
            Direction::Up => Point::new(0, 1),
            Direction::Right => Point::new(1, 0),
            Direction::Left => Point::new(-1, 0),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn new(head: Point, tail: Point) -> Rope {
        Rope { head, tail }
    }
    fn step(&mut self, dir: Direction) {
        self.head = self.head + dir.vector();
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let x = self.head.x - self.tail.x;
        let y = self.head.y - self.tail.y;

        let vec = if x.abs() == 2 {
            Point::new(x / x.abs(), y)
        } else if y.abs() == 2 {
            Point::new(x, y / y.abs())
        } else {
            Point::new(0, 0)
        };

        self.tail = self.tail + vec;
    }
}

fn solve_part_one(input: String) -> i32 {
    let mut visited_positions = HashSet::new();
    let mut rope = Rope::new(Point::new(0, 0), Point::new(0, 0));
    visited_positions.insert(rope.tail);
    for (dir, steps) in parse_input(&input) {
        for _ in 0..steps as usize {
            rope.step(dir);
            visited_positions.insert(rope.tail);
        }
    }
    visited_positions.len() as i32
}

struct ChainRope {
    chain: Vec<Point>,
}

impl ChainRope {
    fn new(length: usize) -> ChainRope {
        ChainRope {
            chain: vec![Point::new(0, 0); length],
        }
    }

    fn step(&mut self, dir: Direction) {
        self.update_head(dir);
        self.update_tail();
    }

    fn update_head(&mut self, dir: Direction) {
        if let Some(head) = self.chain.first_mut() {
            let pt: Point = *head + dir.vector();
            head.x = pt.x;
            head.y = pt.y;
        }
    }

    fn update_tail(&mut self) {
        let mut prev = match self.chain.first() {
            Some(head) => *head,
            None => return,
        };
        for pt in self.chain.iter_mut().skip(1) {
            // let prev = self.chain[i];
            let x = prev.x - pt.x;
            let y = prev.y - pt.y;

            let vec = if x.abs() == 2 && y.abs() == 2 {
                Point::new(x / x.abs(), y / y.abs())
            } else if x.abs() == 2 {
                Point::new(x / x.abs(), y)
            } else if y.abs() == 2 {
                Point::new(x, y / y.abs())
            } else {
                Point::new(0, 0)
            };

            let new_pt = *pt + vec;
            pt.x = new_pt.x;
            pt.y = new_pt.y;

            prev = new_pt.clone();
        }
    }

    fn tail(&self) -> Option<Point> {
        self.chain.last().copied()
    }
}

fn solve_part_two(input: String) -> i32 {
    let mut visited_positions = HashSet::new();
    let mut rope = ChainRope::new(10);
    if let Some(last) = rope.tail() {
        visited_positions.insert(last);
    }
    for (dir, steps) in parse_input(&input) {
        for _ in 0..steps as usize {
            rope.step(dir);
            if let Some(last) = rope.tail() {
                visited_positions.insert(last);
            }
        }
    }
    // print_path(&visited_positions);
    visited_positions.len() as i32
}

#[allow(dead_code)]
fn print_path(points: &HashSet<Point>) -> Option<()> {
    let left_most = points.iter().map(|p| p.x).min()?;
    let right_most = points.iter().map(|p| p.x).max()?;
    let top_most = points.iter().map(|p| p.y).min()?;
    let bottom_most = points.iter().map(|p| p.y).max()?;

    for y in top_most..bottom_most + 1 {
        for x in left_most..right_most + 1 {
            if x == 0 && y == 0 {
                print!("S");
            } else if points.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_example() {
        assert_eq!(solve_part_one(INPUT.into()), 13);
    }

    #[test]
    fn test_example_1_part_2() {
        assert_eq!(solve_part_two(INPUT.into()), 1);
    }
    #[test]
    fn test_example_2_part_2() {
        assert_eq!(solve_part_two(INPUT2.into()), 36);
    }
}
