use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    hands: Vec<(i64, String, i64)>,
    hands_joker: Vec<(i64, String, i64)>,
}
impl Solve for Problem {
    /// Short Description
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for (i, (_hand_type, _hand, bid)) in self.hands.iter().enumerate() {
            sum += bid * (i as i64 + 1);
        }
        sum
    }

    /// Short Description
    #[allow(clippy::cast_possible_wrap)]
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
            let bid = parts[1].parse::<i64>().unwrap();
            let mut hand = parts[0].to_string();
            // change letters so they are ascending in ascii
            hand = hand
                .replace('T', "a")
                .replace('J', "b")
                .replace('Q', "c")
                .replace('K', "d")
                .replace('A', "e");
            let joker_hand = hand.replace('b', "1");
            let hand_type = Self::determine_type(parts[0], false);
            let hand_type_joker = Self::determine_type(&joker_hand, true);

            hands.push((hand_type, hand, bid));

            hands_joker.push((hand_type_joker, joker_hand, bid));
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

        Problem {
            hands,
            hands_joker,
        }
    }

    fn determine_type(hand: &str, joker: bool) -> i64 {
        // Group cards by key and count
        let mut card_counts: HashMap<char, i64> =
            hand.chars().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        // remove the jokers (if they are not the only card)
        let mut joker_count = 0;
        if joker && card_counts.len() > 1 {
            joker_count = card_counts.remove(&'1').unwrap_or(0);
        }

        // Extract the card frequencies and sort descending
        let mut card_frequencies: Vec<i64> = card_counts.values().copied().collect();
        card_frequencies.sort_unstable();
        card_frequencies.reverse();

        // Add back in the jokers to maximize the highest card
        card_frequencies[0] += joker_count;

        match card_frequencies.first().unwrap() {
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
    use crate::load_file;

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\07_test.txt")).p1(), 6440);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\07_test.txt")).p2(), 5905);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
