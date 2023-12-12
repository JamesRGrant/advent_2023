use crate::Solve;

enum Dir {
    U,
    D,
    L,
    R,
    X,
}
const LEFT: [char; 3] = ['┘', '┐', '─'];
const RIGHT: [char; 3] = ['└', '┌', '─'];
const BOTTOM: [char; 3] = ['┌', '┐', '│'];
const TOP: [char; 3] = ['└', '┘', '│'];

pub struct Problem {
    grid: Vec<Vec<char>>,
    start_pos: (usize, usize),
    path: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Find the midpoint of the path
    fn p1(&mut self) -> i64 {
        let sum;

        if LEFT.contains(&self.grid[self.start_pos.0][self.start_pos.1 + 1]) {
            sum = self.follow_path(self.start_pos.1 + 1, self.start_pos.0, Dir::R);
        } else if RIGHT.contains(&self.grid[self.start_pos.0][self.start_pos.1 - 1]) {
            sum = self.follow_path(self.start_pos.1 - 1, self.start_pos.0, Dir::L);
        } else if BOTTOM.contains(&self.grid[self.start_pos.0 - 1][self.start_pos.1]) {
            sum = self.follow_path(self.start_pos.1, self.start_pos.0 - 1, Dir::U);
        } else if TOP.contains(&self.grid[self.start_pos.0 + 1][self.start_pos.1]) {
            sum = self.follow_path(self.start_pos.1, self.start_pos.0 + 1, Dir::D);
        } else {
            panic!("No path found from start");
        }

        if sum % 2 == 0 {
            sum / 2
        } else {
            sum / 2 + 1
        }
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for (row, line) in self.path.iter().enumerate() {
            let mut tmp_line = line.clone();
            let mut inside = false;

            // Get the length of the line minus the padding
            let pipe_length = line.iter().collect::<String>().trim().len();
            let mut start = 0;
            let mut i = 1;
            while i < line.len() {
                if start == 0 && line[i] != ' ' {
                    start = i;
                }
                if line[i] == '│' {
                    inside = !inside;
                } else if line[i] == '┌' {
                    loop {
                        i += 1;
                        if line[i] == '┘' {
                            inside = !inside;
                            break;
                        } else if line[i] == '┐' {
                            break;
                        }
                    }
                } else if line[i] == '└' {
                    loop {
                        i += 1;
                        if line[i] == '┘' {
                            break;
                        } else if line[i] == '┐' {
                            inside = !inside;
                            break;
                        }
                    }
                } else if line[i] == ' ' && inside && self.check_up_down(i, row) {
                    tmp_line[i] = '█';
                    sum += 1;
                }
                i += 1;

                if i >= start + pipe_length {
                    break;
                }
            }

            // Output the line with the sum
            // println!("{}  {sum}",tmp_line.iter().collect::<String>());
        }
        sum
    }
}
impl Problem {
    fn check_up_down(&self, x: usize, y: usize) -> bool {
        let mut i = y - 1;
        let mut horizontal = 0;
        while i > 0 {
            if self.path[i][x] == '─' {
                horizontal += 1;
            } else if self.path[i][x] == '└' {
                loop {
                    i -= 1;
                    if self.path[i][x] == '┐' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == '┌' {
                        break;
                    }
                }
            } else if self.path[i][x] == '┘' {
                loop {
                    i -= 1;
                    if self.path[i][x] == '┌' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == '┐' {
                        break;
                    }
                }
            }
            i -= 1;
        }

        if horizontal == 0 {
            return false;
        }

        horizontal = 0;
        i = y + 1;
        while i < self.path.len() {
            if self.path[i][x] == '─' {
                horizontal += 1;
            } else if self.path[i][x] == '┐' {
                loop {
                    i += 1;
                    if self.path[i][x] == '└' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == '┘' {
                        break;
                    }
                }
            } else if self.path[i][x] == '┌' {
                loop {
                    i += 1;
                    if self.path[i][x] == '┘' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == '└' {
                        break;
                    }
                }
            }
            i += 1;
        }
        horizontal != 0
    }

    fn follow_path(&mut self, x: usize, y: usize, direction: Dir) -> i64 {
        let mut steps: i64 = 1;
        let mut last = self.start_pos;
        let mut walk = (y, x);

        while walk != self.start_pos {
            let cur = self.grid[walk.0][walk.1];
            self.path[walk.0][walk.1] = cur;
            let this_step = walk;
            let mut last_dir = Dir::X;

            // Right
            let test = (walk.0, walk.1 + 1);
            if last != test && RIGHT.contains(&cur) {
                let new = self.grid[test.0][test.1];
                if LEFT.contains(&new) || new == 'S' {
                    walk = test;
                    last_dir = Dir::R;
                }
            }
            // Left
            let test = (walk.0, walk.1 - 1);
            if last != test && (walk == this_step) && LEFT.contains(&cur) {
                let new = self.grid[test.0][test.1];
                if RIGHT.contains(&new) || new == 'S' {
                    walk = test;
                    last_dir = Dir::L;
                }
            }
            // Up
            let test = (walk.0 - 1, walk.1);
            if last != test && (walk == this_step) && TOP.contains(&cur) {
                let new = self.grid[test.0][test.1];
                if BOTTOM.contains(&new) || new == 'S' {
                    walk = test;
                    last_dir = Dir::U;
                }
            }
            // Down
            let test = (walk.0 + 1, walk.1);
            if last != test && (walk == this_step) && BOTTOM.contains(&cur) {
                let new = self.grid[test.0][test.1];
                if TOP.contains(&new) || new == 'S' {
                    walk = test;
                    last_dir = Dir::D;
                }
            }

            // If we did not move, this path is dead
            assert!(walk != this_step, "Followed dead path");

            if walk == self.start_pos {
                // Replace the S with the correct symbol
                self.path[walk.0][walk.1] = match (last_dir, direction) {
                    (Dir::R, Dir::R) | (Dir::L, Dir::L) => '─',
                    (Dir::U, Dir::U) | (Dir::D, Dir::D) => '│',
                    (Dir::R, Dir::U) | (Dir::D, Dir::L) => '┘',
                    (Dir::R, Dir::D) | (Dir::U, Dir::L) => '┐',
                    (Dir::L, Dir::U) | (Dir::D, Dir::R) => '└',
                    (Dir::L, Dir::D) | (Dir::U, Dir::R) => '┌',
                    (_, _) => panic!("Invalid direction"),
                };

                return steps;
            }
            last = this_step;
            steps += 1;
        }
        0
    }

    pub fn new(data: &[String]) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut start_pos: (usize, usize) = (0, 0);

        // 1 space padding around the grid to make out of bounds easier to handle
        let blank_line: Vec<char> = vec![' '; data[0].len() + 2];
        grid.push(blank_line.clone());
        for (i, line) in data.iter().enumerate() {
            // Replace letters with cleaner symbols ┌┘└┐─│
            let mut g: Vec<char> = vec![' '];
            for (j, c) in line.chars().enumerate() {
                g.push(match c {
                    'F' => '┌',
                    'J' => '┘',
                    '7' => '┐',
                    'L' => '└',
                    '-' => '─',
                    '|' => '│',
                    'S' => {
                        start_pos = (i + 1, j + 1);
                        'S'
                    }
                    _ => c,
                });
            }
            g.push(' ');
            grid.push(g);
        }
        grid.push(blank_line);

        // Create a blank grid that will used to store the actual path
        let cols = grid[0].len();
        let rows = grid.len();

        Problem {
            grid,
            start_pos,
            path: vec![vec![' '; cols]; rows],
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
        assert_eq!(Problem::new(&load_file("input\\10_test.txt")).p1(), 8);
        println!("P2 elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2_1() {
        let start = std::time::Instant::now();
        let mut s = Problem::new(&load_file("input\\10_test1.txt"));
        s.p1(); // setup the grid in p1
        assert_eq!(s.p2(), 4);
        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }

    #[test]
    fn p2_2() {
        let start = std::time::Instant::now();
        let mut s = Problem::new(&load_file("input\\10_test2.txt"));
        s.p1(); // setup the grid in p1
        assert_eq!(s.p2(), 8);
        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
    #[test]
    fn p2_3() {
        let start = std::time::Instant::now();
        let mut s = Problem::new(&load_file("input\\10_test3.txt"));
        s.p1(); // setup the grid in p1
        assert_eq!(s.p2(), 10);
        println!("Total elapsed time:    {:>10?}", start.elapsed());
    }
}
