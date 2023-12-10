use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    win_counts: Vec<i64>,
}
impl Solve for Problem {
    /// Return the sum of the geometric progression of wins (1=1, 2=2, 3=4...)
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn p1(&mut self) -> i64 {
        let mut sum = 0;

        for val in &self.win_counts {
            if *val > 0 {
                sum += i64::pow(2, (*val - 1) as u32);
            }
        }

        sum
    }

    /// Wins add that many future cards
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        let mut active_cards: HashMap<usize, i64> = HashMap::new();

        // Start with one of each of the cards
        for i in 0..self.win_counts.len() {
            active_cards.insert(i, 1);
        }

        // Loop through the cards
        for i in 0..self.win_counts.len() {
            // Get the number of cards here, record it
            let card_count = *active_cards.get(&i).unwrap();
            sum += card_count;

            // Add the cards we "won" (note the multiplier...if 5 of the current, add 5)
            for card in (i + 1)..(i + 1 + self.win_counts[i] as usize) {
                active_cards.entry(card).and_modify(|x| *x += card_count);
            }
        }

        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut win_counts: Vec<i64> = Vec::new();
        // Brought these out of the loop because it is slightly faster to clear()
        let mut winning_numbers: Vec<i64> = Vec::new();
        let mut my_numbers: Vec<i64> = Vec::new();

        for line in data {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let mut index = 0;

            // Get the winning numbers
            for (i, t) in tokens.iter().enumerate().skip(2) {
                if *t == "|" {
                    index = i;
                    break;
                }
                winning_numbers.push(t.parse::<i64>().unwrap());
            }
            winning_numbers.sort_unstable();

            // Get my numbers
            for t in tokens.iter().skip(index + 1) {
                my_numbers.push(t.parse::<i64>().unwrap());
            }

            // Count the wins
            let mut wins = 0;
            for val in &my_numbers {
                if winning_numbers.binary_search(val).is_ok() {
                    wins += 1;
                }
            }
            win_counts.push(wins);
            my_numbers.clear();
            winning_numbers.clear();
        }

        Problem {
            win_counts,
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
        assert_eq!(Problem::new(&load_file("input\\04_test.txt")).p1(), 13);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\04_test.txt")).p2(), 30);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
