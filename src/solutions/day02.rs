use core::panic;

use crate::Solve;
pub struct Problem {
    red: Vec<i64>,
    green: Vec<i64>,
    blue: Vec<i64>,
}

impl Solve for Problem {
    #![warn(clippy::cast_possible_wrap)]
    /// Find the games that can be played with the given dice (sum the indices of the games)
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut sum: i64 = 0;

        for i in 0..self.red.len() {
            if self.red[i] <= max_red && self.green[i] <= max_green && self.blue[i] <= max_blue {
                sum += i as i64 + 1;
            }
        }
        sum
    }

    /// Sum the product of the max dice count for each game
    fn p2(&mut self) -> i64 {
        let mut sum: i64 = 0;

        for i in 0..self.red.len() {
            sum += self.red[i] * self.green[i] * self.blue[i];
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut blue = Vec::new();
        let mut red = Vec::new();
        let mut green = Vec::new();
        for line in data {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut tokens = line.split(' ');
            // Throw away "Game 1:"
            tokens.next();
            tokens.next();
            while let Some(num) = tokens.next() {
                let count = num.parse::<i64>().unwrap();
                // Match "red" "red,", "red;", etc.
                match &tokens.next().unwrap()[0..3] {
                    "red" => r = i64::max(r, count),
                    "gre" => g = i64::max(g, count),
                    "blu" => b = i64::max(b, count),
                    _ => panic!("Unexpected color"),
                }
            }
            red.push(r);
            green.push(g);
            blue.push(b);
        }

        Problem {
            red,
            green,
            blue,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_tests() {
        let start = std::time::Instant::now();
        let mut s = Problem::new(&crate::load_file("input\\02_test.txt"));

        assert_eq!(s.p1(), 8);
        assert_eq!(s.p2(), 2286);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
