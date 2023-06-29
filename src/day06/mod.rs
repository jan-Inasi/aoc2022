use queues::*;

pub fn solve(input: String, is_part_one: bool) {
    // benchmark();
    let result = if is_part_one {
        solve_window_size(input, 4)
    } else {
        solve_window_size(input, 14)
    };

    println!("output: {result}");
}

struct Counter {
    count_map: std::collections::HashMap<char, i32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count_map: std::collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, element: char) {
        self.count_map
            .entry(element)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    pub fn sub(&mut self, element: char) {
        self.count_map.entry(element).and_modify(|x| *x -= 1);
        if let Some(count) = self.count_map.get(&element) {
            if *count == 0 {
                self.count_map.remove(&element);
            }
        }
    }

    pub fn n_distinct_elements(&self) -> usize {
        self.count_map.len()
    }
}

fn solve_window_size(input: String, window_size: usize) -> i32 {
    let mut queue = Queue::<char>::new();
    let mut counter = Counter::new();

    for symbol in input
        .chars()
        .filter(|x| x.is_alphabetic())
        .take(window_size)
    {
        if let Err(e) = queue.add(symbol) {
            println!("WARNING error while adding on queue\n{e}");
        }
        counter.add(symbol);
    }

    for (i, symbol) in input
        .chars()
        .filter(|x| x.is_alphabetic())
        .skip(window_size)
        .enumerate()
    {
        if counter.n_distinct_elements() == window_size {
            return i as i32 + window_size as i32;
        }

        match queue.remove() {
            Ok(elem) => {
                counter.sub(elem);
            }
            Err(err) => {
                println!("WARNING error while removing from queue\n{err}");
            }
        }
        counter.add(symbol);
        if let Err(err) = queue.add(symbol) {
            println!("WARNING error while adding on queue\n{err}");
        };
    }
    -1
}

#[allow(dead_code)]
fn alternative_solve_window_size(input: String, window_size: usize) -> i32 {
    let input_len = input.len();
    for i in 0..(input_len - window_size) {
        let window = &input[i..i + window_size];
        let set: std::collections::HashSet<_> = window.chars().collect();
        if set.len() == window_size {
            return (i + window_size) as i32;
        }
    }
    -1
}

#[allow(dead_code)]
const SIZE: usize = 10_000_000;
#[allow(dead_code)]
fn benchmark() {
    let input: String = ['a'; SIZE].iter().collect();
    let window_size = 40;

    let start = std::time::SystemTime::now();
    solve_window_size(input, window_size);
    let duration = start.elapsed().expect("sth went wrong");
    println!("counter: {}", duration.as_millis());

    let input: String = ['a'; SIZE].iter().collect();
    let start = std::time::SystemTime::now();
    alternative_solve_window_size(input, window_size);
    let duration = start.elapsed().expect("sth went wrong");
    println!("window : {}", duration.as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let t1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let t2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let t3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let t4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvj";

        assert_eq!(solve_window_size(t1.into(), 4), 5);
        assert_eq!(solve_window_size(t2.into(), 4), 6);
        assert_eq!(solve_window_size(t3.into(), 4), 10);
        assert_eq!(solve_window_size(t4.into(), 4), 11);
    }

    #[test]
    fn test_example_alternative() {
        let t1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let t2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let t3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let t4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvj";

        assert_eq!(alternative_solve_window_size(t1.into(), 4), 5);
        assert_eq!(alternative_solve_window_size(t2.into(), 4), 6);
        assert_eq!(alternative_solve_window_size(t3.into(), 4), 10);
        assert_eq!(alternative_solve_window_size(t4.into(), 4), 11);
    }
}
