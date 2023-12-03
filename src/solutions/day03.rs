use crate::Solve;

pub struct Problem {
    nums: Vec<(i64, usize, usize, usize)>,
    symbols: Vec<(usize, usize)>,
    gears: Vec<(usize, usize)>,
}
impl Solve for Problem {
    /// Add any number adjacent to a symbol
    fn p1(&mut self) -> i64 {
        let mut sum: i64 = 0;

        for (n, x1, x2, y1) in &self.nums {
            // Search +/- 1 from the bounds of the number
            for col in (*x1 - 1)..=(*x2 + 1) {
                // Binary search is faster than contains() (4ms to 0.3ms), OK() if found
                if self.symbols.binary_search(&(col, *y1 - 1)).is_ok()
                    || self.symbols.binary_search(&(col, *y1)).is_ok()
                    || self.symbols.binary_search(&(col, *y1 + 1)).is_ok()
                {
                    sum += *n;
                    break;
                }
            }
        }
        sum
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut sum: i64 = 0;
        let mut adj_nums: Vec<i64> = Vec::new();
        for (x, y) in &self.gears {
            for (n, x1, x2, y1) in &self.nums {
                if Problem::check_num(*x1, *y1, *x2, *y1, *x, *y) {
                    adj_nums.push(*n);
                }
            }
            if adj_nums.len() == 2 {
                sum += adj_nums[0] * adj_nums[1];
            }
            adj_nums.clear();
        }
        sum
    }
}
impl Problem {
    /// Check if a number is adjacent to a gear
    fn check_num(x1: usize, y1: usize, x2: usize, y2: usize, x: usize, y: usize) -> bool {
        y1 <= (y + 1) && y2 >= (y - 1) && x1 <= (x + 1) && x2 >= (x - 1)
    }

    pub fn new(input: Vec<String>) -> Self {
        let len = input[0].len();
        let mut nums: Vec<(i64, usize, usize, usize)> = Vec::new();
        let mut symbols: Vec<(usize, usize)> = Vec::new();
        let mut gears: Vec<(usize, usize)> = Vec::new();

        for (row, line) in input.iter().enumerate() {
            let mut num = String::new();
            for (col, char) in line.chars().enumerate() {
                // We're going to always shift the coordinates by 1 so we don't get out of bounds on zero
                // So, row + 1 and col + 1
                let r = row + 1;
                let c = col + 1;

                if char.is_ascii_digit() {
                    num.push(char);
                } else {
                    if !num.is_empty() {
                        let n = num.parse::<i64>().unwrap();
                        nums.push((n, c - num.len(), c - 1, r));
                        num.clear();
                    }
                    if char != '.' {
                        symbols.push((c, r));
                        if char == '*' {
                            gears.push((c, r));
                        }
                    }
                }
            }
            if !num.is_empty() {
                // remember, modified for row + 1, col + 1
                let n = num.parse::<i64>().unwrap();
                nums.push((n, len - num.len() + 1, len, row + 1));
                num.clear();
            }
        }

        // Required for binary search
        symbols.sort_unstable();

        Problem {
            nums,
            symbols,
            gears,
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
        let buf = BufReader::new(File::open("input\\03_test.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();
        let mut s = Problem::new(data);

        assert_eq!(s.p1(), 4361);
        assert_eq!(s.p2(), 467_835);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
