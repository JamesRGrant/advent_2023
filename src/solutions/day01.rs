use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            let mut digit = "".to_string();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    digit.push(c);
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_ascii_digit() {
                    digit.push(c);
                    break;
                }
            }

            sum += digit.parse::<i64>().unwrap();
        }

        sum
    }

    fn p2(&mut self) -> i64 {
        let nums: Vec<&str> =
            ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
                .to_vec();
        let ascii_nums: Vec<char> = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].to_vec();
        let mut sum = 0;

        for line in &self.data {
            let mut digit = "".to_string();

            // Get the first number
            'outer: for (i, c) in line.chars().enumerate() {
                // Is it a digit?
                if c.is_ascii_digit() {
                    digit.push(c);
                    break;
                }
                // Is it a word?
                for (cnt, n) in nums.iter().enumerate() {
                    if line[i..].starts_with(n) {
                        digit.push(ascii_nums[cnt]);
                        break 'outer;
                    }
                }
            }

            // Get the last number
            let len = line.len();
            'outer: for (i, c) in line.chars().rev().enumerate() {
                // Is it a digit?
                if c.is_ascii_digit() {
                    digit.push(c);
                    break;
                }
                // Is it a word?
                if i >= 2 {
                    for (cnt, n) in nums.iter().enumerate() {
                        if line[(len - i - 1)..].starts_with(n) {
                            digit.push(ascii_nums[cnt]);
                            break 'outer;
                        }
                    }
                }
            }

            sum += digit.parse::<i64>().unwrap();
        }

        sum
    }
}
impl Problem {
    pub fn new(data: Vec<String>) -> Self {
        Problem {
            data,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn run_tests() {
        let start = std::time::Instant::now();

        // This problem has two test inputs, so lets do ths manually
        let p1_test: Vec<String> = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        let mut s = Problem::new(p1_test);
        assert_eq!(s.p1(), 142);

        let mut s = Problem::new(crate::load_file("input\\01_test.txt"));
        assert_eq!(s.p2(), 281);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
