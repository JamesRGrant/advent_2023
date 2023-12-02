use crate::Solve;
pub struct Problem {
    red: Vec<i64>,
    green: Vec<i64>,
    blue: Vec<i64>,
}

impl Solve for Problem {
    #![warn(clippy::cast_possible_wrap)]
    /// Find the games that can be played with the given dice (sum the indices of the games)
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
    pub fn new(input: Vec<String>) -> Self {
        let mut blue = Vec::new();
        let mut red = Vec::new();
        let mut green = Vec::new();
        for line in input {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let s1 = line.split(':').collect::<Vec<&str>>();
            let rounds = s1[1].split(';').collect::<Vec<&str>>();
            for round in rounds {
                let dice = round.split(',').collect::<Vec<&str>>();
                for die in dice {
                    let count = die.trim().split(' ').collect::<Vec<&str>>();
                    let v = count[0].parse::<i64>().unwrap();
                    match count[1] {
                        "red" => r = i64::max(r, v),
                        "green" => g = i64::max(g, v),
                        "blue" => b = i64::max(b, v),
                        _ => (),
                    }
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
        let mut s = Problem::new(crate::load_file("input\\02_test.txt"));

        assert_eq!(s.p1(), 8);
        assert_eq!(s.p2(), 2286);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
