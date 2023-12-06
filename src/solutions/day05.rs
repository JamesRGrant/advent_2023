use crate::Solve;

pub struct Problem {
    seeds: Vec<i64>,
    transforms: Vec<Vec<(i64, i64, i64)>>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut min = i64::MAX;

        for val in &self.seeds {
            let mut tmp = *val;
            for mapping in &self.transforms {
                for (start, end, offset) in mapping {
                    if tmp >= *start && tmp <= *end {
                        tmp += *offset;
                        break;
                    }
                }
            }
            min = i64::min(min, tmp);
        }

        min
    }

    /// Rather than try each number of input, we track the ranges and offsets
    /// The ranges need to be split at each stage of the transform into multiple ranges
    fn p2(&mut self) -> i64 {
        let mut min = i64::MAX;

        for (i, start) in self.seeds.iter().enumerate().step_by(2) {
            // load the initial range for this seed
            let mut input_ranges: Vec<(i64, i64)> =
                [(*start, start + self.seeds[i + 1] - 1)].to_vec();

            // Loop through the transform stages
            for mapping in &self.transforms {
                let mut new_ranges: Vec<(i64, i64)> = Vec::new();
                let end_mapping_index = mapping.len() - 1;

                // Loop through each input range
                for (mut r_start, r_end) in input_ranges {
                    // Loop through each transform range (assumes they are sorted by start)
                    for (i, (start, end, offset)) in mapping.iter().enumerate() {
                        // Case 0: before, ignore (END CASE if only/last item)
                        if *end < r_start {
                            if i == end_mapping_index {
                                new_ranges.push((r_start, r_end));
                            }
                        }
                        // Case 1: overlapping start
                        else if *start <= r_start && *end >= r_start && *end < r_end {
                            new_ranges.push((r_start + *offset, *end + *offset));
                            r_start = *end + 1;
                            if i == end_mapping_index {
                                new_ranges.push((r_start, r_end));
                            }
                        }
                        // Case 2: in the middle: 3 splits
                        else if *start > r_start && *end < r_end {
                            new_ranges.push((r_start, *start - 1));
                            new_ranges.push((*start + *offset, *end + *offset));
                            r_start = *end + 1;
                            if i == end_mapping_index {
                                new_ranges.push((r_start, r_end));
                            }
                        }
                        // Case 3: overlapping top end: 2 splits (END CASE)
                        else if *start > r_start && *start <= r_end && *end >= r_end {
                            new_ranges.push((r_start, *start - 1));
                            new_ranges.push((*start + *offset, r_end + *offset));
                            break;
                        }
                        // Case 4: total overlap (END CASE)
                        else if *start <= r_start && *end >= r_end {
                            new_ranges.push((r_start + *offset, r_end + *offset));
                            break;
                        }
                        // Case 5: completely past (END CASE)
                        else if *start > r_end {
                            new_ranges.push((r_start, r_end));
                            break;
                        } else {
                            panic!("Unhandled case: current range: {r_start} {r_end}   transform range: {start} {end} {offset}");
                        }
                    }
                }
                input_ranges = new_ranges;
            }

            // Find the min of the new ranges
            for (start, _end) in &input_ranges {
                min = i64::min(min, *start);
            }
        }

        // This code works in 109 s
        // for (i, start) in self.seeds.iter().enumerate().step_by(2) {
        //     for val in *start..(start + self.seeds[i + 1]) {
        //         let mut tmp = val;
        //         for mapping in &self.transforms {
        //             for (start, end, offset) in mapping {
        //                 if tmp >= *start && tmp <= *end {
        //                     tmp += *offset;
        //                     break;
        //                 }
        //             }
        //         }
        //         min = i64::min(min, tmp);
        //     }
        // }

        min
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut transforms: Vec<Vec<(i64, i64, i64)>> = Vec::new();
        transforms.push(Vec::new());
        let mut vec_index = 0;

        let seeds: Vec<i64> =
            data[0].split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();

        for line in data.iter().skip(3) {
            if !line.is_empty() {
                if line.chars().next().unwrap().is_ascii_digit() {
                    let nums: Vec<i64> =
                        line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                    transforms[vec_index].push((nums[1], nums[1] + nums[2] - 1, nums[0] - nums[1]));
                } else {
                    transforms.push(Vec::new());
                    vec_index += 1;
                }
            }
        }

        // Sort the transforms (required for part 2)
        for t in &mut transforms {
            t.sort_unstable();
        }

        Problem {
            seeds,
            transforms,
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
        let buf = BufReader::new(File::open("input\\05_test.txt").expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();
        let mut s = Problem::new(&data);

        assert_eq!(s.p1(), 35);
        assert_eq!(s.p2(), 46);

        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
