use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    hands: Vec<(i64, String, i64)>,
    hands_joker: Vec<(i64, String, i64)>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for (i, (_hand_type, _hand, bid)) in self.hands.iter().enumerate() {
            sum += bid * (i as i64 + 1);
        }
        sum
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for (i, (_hand_type, _hand, bid)) in self.hands_joker.iter().enumerate() {
            sum += bid * (i as i64 + 1);
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut hands: Vec<(i64, String, i64)> = Vec::new();
        let mut hands_joker: Vec<(i64, String, i64)> = Vec::new();
        for line in data {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut hand = parts[0].to_string();
            // change letters so they are ascending in ascii
            hand = hand
                .replace('T', "a")
                .replace('J', "b")
                .replace('Q', "c")
                .replace('K', "d")
                .replace('A', "e");
            let hand_joker = hand.replace('b', "1");
            let bid = parts[1].parse::<i64>().unwrap();
            let hand_type = Self::determine_type(parts[0], false);
            let hand_type_joker = Self::determine_type(&hand_joker, true);

            hands.push((hand_type, hand, bid));

            hands_joker.push((hand_type_joker, hand_joker, bid));
        }

        hands.sort_unstable_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        hands_joker.sort_unstable_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        // println!("{hands:?}");
        // println!("{hands_joker:?}");
        // hands.reverse();
        // println!("{hands:?}");

        Problem {
            hands,
            hands_joker,
        }
    }

    fn determine_type(hand: &str, joker: bool) -> i64 {
        let mut card_counts: HashMap<char, i64> =
            hand.chars().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        let mut joker_count = 0;
        if joker {
            // remove the jokers (if they are not the only card)
            if card_counts.len() > 1 {
                joker_count = card_counts.remove(&'1').unwrap_or(0);
            }
        }
        let mut card_frequencies: Vec<i64> = card_counts.values().copied().collect();
        card_frequencies.sort_unstable();
        card_frequencies.reverse();

        // add in the joker (just assume the max for now)
        card_frequencies[0] += joker_count;

        match card_frequencies[0] {
            5 => 7, // 5 of a kind
            4 => 6, // 4 of a kind
            3 => {
                if card_frequencies[1] == 2 {
                    5 // Full House
                } else {
                    4 // 3 of a kind
                }
            }
            2 => {
                if card_frequencies[1] == 2 {
                    3 // 2 pair
                } else {
                    2 // 1 pair
                }
            }
            _ => 1, // High Card
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
        let buf = BufReader::new(File::open("input\\07_test.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();
        let mut s = Problem::new(&data);

        assert_eq!(s.p1(), 6440);
        assert_eq!(s.p2(), 5905);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
