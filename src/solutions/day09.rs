use crate::Solve;

pub struct Problem {
    data: Vec<Vec<i64>>,
    neg: i64,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for row in &self.data {
            let (pos, neg) = Problem::process_line(row);
            sum += pos;
            self.neg += neg;
        }
        sum
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        self.neg
    }
}
impl Problem {
    pub fn process_line(data: &Vec<i64>) -> (i64, i64) {
        let mut diffs: Vec<i64> = Vec::new();
        if *data.iter().min().unwrap() == 0 && *data.iter().max().unwrap() == 0 {
            return (0, 0);
        }
        for i in 1..data.len() {
            diffs.push(data[i] - data[i - 1]);
        }

        let (pos, neg) = Problem::process_line(&diffs);
        (pos + data.iter().last().unwrap(), data.iter().next().unwrap() - neg)
    }

    pub fn new(data: &[String]) -> Self {
        let data: Vec<Vec<i64>> = data
            .iter()
            .map(|l| l.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect())
            .collect();
        Problem {
            data,
            neg: 0,
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
        assert_eq!(Problem::new(&load_file("input\\09_test.txt")).p1(), 114);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        let mut p = Problem::new(&load_file("input\\09_test.txt"));
        p.p1();
        assert_eq!(p.p2(), 2);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
