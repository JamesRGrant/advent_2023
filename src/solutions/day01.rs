use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            let mut digit = String::new();
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
            let mut digit = String::new();

            // Get the first number
            'outer: for (i, c) in line.chars().enumerate() {
                // Is it a digit?
                if c.is_ascii_digit() {
                    digit.push(c);
                    break;
                }
                // Is it a word?
                for (n, a) in nums.iter().zip(ascii_nums.iter()) {
                    if line[i..].starts_with(n) {
                        digit.push(*a);
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
                    for (n, a) in nums.iter().zip(ascii_nums.iter()) {
                        if line[(len - i - 1)..].starts_with(n) {
                            digit.push(*a);
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
    pub fn new(data: &[String]) -> Self {
        Problem {
            data: data.to_vec(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\01_test.txt")).p1(), 142);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\01_test2.txt")).p2(), 281);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
