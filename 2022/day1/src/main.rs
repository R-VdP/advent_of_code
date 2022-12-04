use std::fs;
use std::str::FromStr;

struct Acc {
    max: Vec<u64>,
    running_calories: u64,
}

impl Acc {
    fn add_running_calories(self, new_calories: u64) -> Self {
        Self {
            running_calories: self.running_calories + new_calories,
            ..self
        }
    }

    fn find_new_max(self) -> Self {
        let mut max = self.max;
        max.push(self.running_calories);
        max.sort_unstable();
        let max_new: Vec<_> = max.into_iter().skip(1).collect();
        assert!(max_new.len() == 3);
        Acc {
            max: max_new,
            running_calories: 0,
        }
    }
}

impl Default for Acc {
    fn default() -> Self {
        Self {
            max: vec![0, 0, 0],
            running_calories: 0,
        }
    }
}

fn main() {
    let input = get_input();
    let lines = input.lines().collect::<Vec<&str>>();

    let acc: Acc = lines
        .iter()
        .fold(Default::default(), |acc, line| match u64::from_str(line) {
            Ok(new_calories) => acc.add_running_calories(new_calories),
            Err(_) => acc.find_new_max(),
        });

    println!("{:?}", acc.max.iter().sum::<u64>())
}

fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap_or_default()
}
