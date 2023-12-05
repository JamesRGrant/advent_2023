use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    nums: HashMap<usize, Vec<(i64, usize, usize)>>,
    symbols: HashMap<usize, Vec<usize>>,
    gears: Vec<(usize, usize)>,
}
impl Solve for Problem {
    /// Add any number adjacent to a symbol
    fn p1(&mut self) -> i64 {
        let mut sum: i64 = 0;

        // loop through nums (a Vec by row)
        for (y1, numbers) in &self.nums {
            for (n, x1, x2) in numbers {
                if self.is_symbol_adjacent(*y1 - 1, *x1, *x2)
                    || self.is_symbol_adjacent(*y1, *x1, *x2)
                    || self.is_symbol_adjacent(*y1 + 1, *x1, *x2)
                {
                    sum += *n;
                }
            }
        }
        sum
    }

    /// For each gear with exactly 2 adjacent numbers, multiply them together
    fn p2(&mut self) -> i64 {
        let mut sum: i64 = 0;
        let mut adj_nums: Vec<i64> = Vec::new();

        for (x, y) in &self.gears {
            for row in (y - 1)..=(y + 1) {
                if let Some(numbers) = self.nums.get(&row) {
                    for (n, x1, x2) in numbers {
                        if Problem::is_num_adjacent(row, *x1, *x2, *x, *y) {
                            adj_nums.push(*n);
                        }
                    }
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
    fn is_num_adjacent(y1: usize, x1: usize, x2: usize, x: usize, y: usize) -> bool {
        y1 <= (y + 1) && y1 >= (y - 1) && x1 <= (x + 1) && x2 >= (x - 1)
    }

    fn is_symbol_adjacent(&self, y: usize, x1: usize, x2: usize) -> bool {
        if let Some(symbols_x) = self.symbols.get(&y) {
            for x in symbols_x {
                if *x >= x1 - 1 && *x <= x2 + 1 {
                    return true;
                }
            }
        }
        false
    }

    pub fn new(data: &[String]) -> Self {
        let len = data[0].len();
        let mut nums: HashMap<usize, Vec<(i64, usize, usize)>> = HashMap::new();
        let mut symbols: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut gears: Vec<(usize, usize)> = Vec::new();
        let mut row = 1;

        // We're going to always shift the coordinates by 1 so we don't get out of bounds on zero
        for line in data {
            let mut num = String::new();
            let mut line_nums = Vec::new();
            let mut line_symbols = Vec::new();
            let mut col = 1;

            for char in line.chars() {
                if char.is_ascii_digit() {
                    num.push(char);
                } else {
                    if !num.is_empty() {
                        let n = num.parse::<i64>().unwrap();
                        line_nums.push((n, col - num.len(), col - 1));
                        num.clear();
                    }
                    if char != '.' {
                        line_symbols.push(col);
                        if char == '*' {
                            gears.push((col, row));
                        }
                    }
                }
                col += 1;
            }
            if !num.is_empty() {
                let n = num.parse::<i64>().unwrap();
                nums.entry(row + 1).or_default().push((n, len - num.len() + 1, len));
                num.clear();
            }
            if !line_nums.is_empty() {
                nums.insert(row, line_nums);
            }
            if !line_symbols.is_empty() {
                symbols.insert(row, line_symbols);
            }
            row += 1;
        }

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
        let mut s = Problem::new(&data);

        assert_eq!(s.p1(), 4361);
        assert_eq!(s.p2(), 467_835);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
