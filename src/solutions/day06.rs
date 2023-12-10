use crate::Solve;

pub struct Problem {
    times: Vec<i64>,
    distances: Vec<i64>,
    time: i64,
    distance: i64,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut product = 1;
        for (i, time) in self.times.iter().enumerate() {
            product *= Problem::algebra(*time, self.distances[i]);
        }
        product
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        Problem::algebra(self.time, self.distance)
    }
}
impl Problem {
    // Solves (T - t) * t > D, where T is the total time, t is the time spent charging, and D is the distance.
    // This is t^2 - Tt + D > 0
    // This is a quadratic equation, so we can use the quadratic formula to solve for t.
    // The quadratic formula is (-b +- sqrt(b^2 - 4ac)) / 2a
    // In this case, a = 1, b = -T, and c = D
    // We can simplify this to (-(-T) +- sqrt((-T)^2 - 4(1)(D))) / 2(1)
    // This is (T +- sqrt(T^2 - 4D)) / 2
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn algebra(time: i64, distance: i64) -> i64 {
        let term = f64::sqrt((time.pow(2) - 4 * distance) as f64);
        let mut min = (time as f64 - term) / 2.0;
        let mut max = (time as f64 + term) / 2.0;
        if min.fract() == 0.0 {
            min += 1.0;
        } else {
            min = min.ceil();
        }
        if max.fract() == 0.0 {
            max -= 1.0;
        } else {
            max = max.floor();
        }
        (max - min + 1.0) as i64
    }
    pub fn new(data: &[String]) -> Self {
        Problem {
            times: data[0].split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap()).collect(),
            distances: data[1]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
            time: data[0][data[0].find(':').unwrap() + 1..]
                .replace(' ', "")
                .parse::<i64>()
                .unwrap(),
            distance: data[1][data[1].find(':').unwrap() + 1..]
                .replace(' ', "")
                .parse::<i64>()
                .unwrap(),
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
        assert_eq!(Problem::new(&load_file("input\\06_test.txt")).p1(), 288);
        println!("P1 elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2() {
        let start = std::time::Instant::now();
        assert_eq!(Problem::new(&load_file("input\\06_test.txt")).p2(), 71503);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }
}
