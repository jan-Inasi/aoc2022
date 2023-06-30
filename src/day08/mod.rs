use std::ops::Index;

pub fn solve(input: String, is_part_one: bool) {
    let result = if is_part_one {
        solve_part_one(parse_input(input))
    } else {
        solve_part_two(parse_input(input))
    };

    println!("output: {result}");
}

fn solve_part_one(forest: Forest) -> i32 {
    let mut count_visible = 0;
    for x in 1..forest.width() - 1 {
        for y in 1..forest.height() - 1 {
            if forest.is_tree_visible((x, y)) {
                count_visible += 1;
            }
        }
    }
    count_visible + forest.width() as i32 * 2 + forest.height() as i32 * 2 - 4
}

fn solve_part_two(forest: Forest) -> i32 {
    let mut max_scenic_score = 0;
    for x in 0..forest.width() {
        for y in 0..forest.height() {
            let score = forest.calc_tree_scenic_score((x, y));
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    max_scenic_score
}

fn parse_input(input: String) -> Forest {
    let mut memory = Vec::<u8>::new();
    let mut height = 0;
    for line in input.lines() {
        height += 1;
        for digit in line.chars() {
            match digit.to_digit(10) {
                Some(tree_height) => memory.push(tree_height as u8),
                None => println!("WARNING couldn't convert '{digit}' to a tree height"),
            }
        }
    }
    let width = memory.len() / height;
    Forest {
        shape: (width as u32, height as u32),
        memory,
    }
}

struct Forest {
    pub shape: (u32, u32),
    pub memory: Vec<u8>,
}

impl Forest {
    pub fn width(&self) -> u32 {
        self.shape.0
    }

    pub fn height(&self) -> u32 {
        self.shape.1
    }

    pub fn calc_idx(&self, point: (u32, u32)) -> usize {
        let (x, y) = point;
        x as usize + y as usize * self.width() as usize
    }

    pub fn is_tree_visible(&self, point: (u32, u32)) -> bool {
        let (x, y) = point;
        let height = self[point];

        if (0..x).all(|xx| self[(xx, y)] < height) {
            true // left visibility
        } else if (x + 1..self.width()).all(|xx| self[(xx, y)] < height) {
            true // right visibility
        } else if (0..y).all(|yy| self[(x, yy)] < height) {
            true // top visibility
        } else if (y + 1..self.height()).all(|yy| self[(x, yy)] < height) {
            true // bottom visibility
        } else {
            false
        }
    }

    pub fn calc_tree_scenic_score(&self, point: (u32, u32)) -> i32 {
        let (x, y) = point;
        let height = self[point];
        let mut left_count = (0..x)
            .rev()
            .take_while(|&xx| self[(xx, y)] < height)
            .count();
        let mut top_count = (0..y)
            .rev()
            .take_while(|&yy| self[(x, yy)] < height)
            .count();
        let mut right_count = (x + 1..self.width())
            .take_while(|&xx| self[(xx, y)] < height)
            .count();
        let mut bottom_count = (y + 1..self.height())
            .take_while(|&yy| self[(x, yy)] < height)
            .count();
        left_count += if left_count == x as usize { 0 } else { 1 };
        right_count += if right_count == (self.width() - x - 1) as usize {
            0
        } else {
            1
        };
        top_count += if top_count == y as usize { 0 } else { 1 };
        bottom_count += if bottom_count == (self.height() - y - 1) as usize {
            0
        } else {
            1
        };
        (left_count * top_count * right_count * bottom_count) as i32
    }
}

impl Index<(u32, u32)> for Forest {
    type Output = u8;

    fn index(&self, point: (u32, u32)) -> &Self::Output {
        &self.memory[self.calc_idx(point)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        assert_eq!(solve_part_one(parse_input(INPUT.into())), 21);
    }
    #[test]
    fn test_example_part_two() {
        assert_eq!(solve_part_two(parse_input(INPUT.into())), 8);
    }

    const INPUT: &str = "30373
25512
65332
33549
35390";
}
