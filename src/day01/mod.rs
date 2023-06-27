struct TopKTracker {
    k: i32,
    topk: Vec<i32>,
}

impl TopKTracker {
    fn new(k: i32) -> Self {
        TopKTracker { k, topk: vec![] }
    }

    fn get_topk(&self) -> &[i32] {
        &self.topk
    }

    pub fn update(&mut self, given_number: i32) -> bool {
        let mut working_number = given_number;
        let mut updated = false;
        for i in 0..self.k as usize {
            if self.topk.len() == i {
                self.topk.push(working_number);
                return true;
            }
            if self.topk[i] < working_number {
                let temp = self.topk[i];
                self.topk[i] = working_number;
                working_number = temp;
                updated = true;
            }
        }
        updated
    }
}

pub fn solve(input: String, is_part_one: bool) -> i32 {
    let topk = if is_part_one { 1 } else { 3 };
    let mut current_calories = 0;
    let mut tracker = TopKTracker::new(topk);
    for line in input.split('\n') {
        match line.parse::<i32>() {
            Result::Ok(number) => current_calories += number,
            Result::Err(..) => {
                tracker.update(current_calories);
                current_calories = 0;
            }
        }
    }
    tracker.get_topk().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

        assert_eq!(solve(input.into(), true), 24000);
    }

    #[test]
    fn test_example_part_two() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

        assert_eq!(solve(input.into(), false), 45000);
    }

    #[test]
    fn test_top_k_tracker() {
        let mut tracker = TopKTracker::new(4);

        for num in [15, -4, 8, 13, 26, 8, 13, 3].iter() {
            tracker.update(*num);
        }

        assert_eq!(tracker.get_topk().len(), 4);
        for (&n, gt) in std::iter::zip(tracker.get_topk(), [26, 15, 13, 13]) {
            assert_eq!(n, gt);
        }

        tracker = TopKTracker::new(4);
        for num in [15, -4].iter() {
            tracker.update(*num);
        }
        assert_eq!(tracker.get_topk().len(), 2);
        for (&n, gt) in std::iter::zip(tracker.get_topk(), [15, -4]) {
            assert_eq!(n, gt);
        }

        tracker = TopKTracker::new(3);
        for num in [6000, 4000, 11000, 24000, 10000].iter() {
            tracker.update(*num);
        }
        assert_eq!(tracker.get_topk().len(), 3);
        for (&n, gt) in std::iter::zip(tracker.get_topk(), [24000, 11000, 10000]) {
            assert_eq!(n, gt);
        }
    }
}
