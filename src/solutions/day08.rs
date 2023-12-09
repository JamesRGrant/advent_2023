use crate::Solve;
use std::collections::HashMap;
use std::io::{self, Write};

pub struct Problem {
    directions: Vec<char>,
    maps: HashMap<String, (String, String)>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        self.count_steps("AAA".to_string(), true)
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        // let  keys: Vec<String> = Vec::new();
        let mut steps: Vec<i64> = Vec::new();

        // load the starting keys
        for key in self.maps.keys() {
            if key.chars().nth(2).unwrap() == 'A' {
                println!("Key: {key}");
                io::stdout().flush().unwrap();

                let step_cnt = self.count_steps(key.clone(), false);
                println!("   Steps: {step_cnt}");
                steps.push(step_cnt);
            }
        }

        Problem::lcm(steps)
    }
}
impl Problem {
    pub fn lcm(nums: Vec<i64>) -> i64 {
        let mut tmp = nums.clone();

        loop {
            // println!("{:?}", tmp);
            let mut min = i64::MAX;
            {
                min = *tmp.iter().min().unwrap();
                if min == *tmp.iter().max().unwrap() {
                    break;
                }
            }

            // increment the smallest
            for (i, num) in tmp.iter_mut().enumerate() {
                if *num == min {
                    *num += nums[i];
                    break;
                }
            }
        }

        tmp[0]
    }

    pub fn count_steps(&self, start: String, zzz: bool) -> i64 {
        let mut key = start;
        let mut step_count = 0;
        let mut i = 0;

        while i <= self.directions.len() {
            // println!("step = {step_count}, i = {i}, key = {key}");
            if step_count > 100000 {
                return -1;
            }

            let (left, right) = self.maps.get(&key).unwrap();
            if self.directions[i] == 'L' {
                key = left.to_string();
            } else {
                key = right.to_string();
            }

            step_count += 1;

            if key == "ZZZ" && zzz {
                break;
            } else {
                let mut flag = true;
                if key.chars().nth(2).unwrap() != 'Z' {
                    flag = false;
                }
                if flag {
                    break;
                }
            }
            if i == self.directions.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }
        step_count
    }

    pub fn new(data: &[String]) -> Self {
        let directions: Vec<char> = data[0].chars().collect();
        let mut maps: HashMap<String, (String, String)> = HashMap::new();

        for line in data.iter().skip(2) {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let source = tokens[0].to_string();
            let left = tokens[2].to_string().replace(['(', ','], "");
            let right = tokens[3].to_string().replace(')', "");

            maps.insert(source, (left, right));
        }

        Problem {
            directions,
            maps,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn run_tests() {
        let start = std::time::Instant::now();
        let buf = BufReader::new(File::open("input\\08_test.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();
        let mut s = Problem::new(&data);

        assert_eq!(s.p1(), 6);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn test2() {
        let start = std::time::Instant::now();
        let buf = BufReader::new(File::open("input\\08_test2.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();

        let mut s = Problem::new(&data);
        assert_eq!(s.p2(), 6);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
