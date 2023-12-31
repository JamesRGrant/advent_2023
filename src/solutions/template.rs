use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        self.data.len() as i64
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        0
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
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn p1() {
        let start = std::time::Instant::now();
        let buf = BufReader::new(File::open("input\\_test.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();
        let mut s = Problem::new(&data);

        assert_eq!(s.p1(), 0);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        let buf = BufReader::new(File::open("input\\_test.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();
        let mut s = Problem::new(&data);

        assert_eq!(s.p2(), 0);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
