use crate::Solve;

pub struct Problem {
    grid: Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    path: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut sum = 0;

        // Check right
        let tmp = self.grid[self.start_row][self.start_col + 1];
        // println!("{tmp}");
        if tmp == '-' || tmp == 'J' || tmp == '7' {
            // println!("Right");
            sum = sum.max(self.follow_path(self.start_col + 1, self.start_row));
        } else {
            // Check left
            let tmp = self.grid[self.start_row][self.start_col - 1];
            // println!("{tmp}");
            if tmp == '-' || tmp == 'L' || tmp == 'F' {
                // println!("Left");
                sum = sum.max(self.follow_path(self.start_col - 1, self.start_row));
            } else {
                // Check up
                let tmp = self.grid[self.start_row - 1][self.start_col];
                // println!("{tmp}");
                if tmp == '|' || tmp == '7' || tmp == 'F' {
                    // println!("Up");
                    sum = sum.max(self.follow_path(self.start_col, self.start_row - 1));
                } else {
                    // Check down
                    let tmp = self.grid[self.start_row + 1][self.start_col];
                    // println!("{tmp}");
                    if tmp == '|' || tmp == 'J' || tmp == 'L' {
                        // println!("Down");
                        sum = sum.max(self.follow_path(self.start_col, self.start_row + 1));
                    }
                }
            }
        }

        // println!("Sum: {}", sum);

        // for row in &self.path {
        //     for c in row {
        //         print!("{c}");
        //     }
        //     println!();
        // }

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
                if line[i] == '|' {
                    inside = !inside;
                } else if line[i] == 'F' {
                    loop {
                        i += 1;
                        if line[i] == 'J' {
                            inside = !inside;
                            break;
                        } else if line[i] == '7' {
                            break;
                        }
                    }
                } else if line[i] == 'L' {
                    loop {
                        i += 1;
                        if line[i] == 'J' {
                            break;
                        } else if line[i] == '7' {
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
            // for c in line.iter().skip(1).take(line.len() - 2) {
            //     if *c == '|' {
            //         inside = !inside;
            //     } else if inside {
            //         sum += 1;
            //     }
            // }
            // let tmp = tmp_line
            //     .iter()
            //     .collect::<String>()
            //     .replace('L', "\\")
            //     .replace('J', "/")
            //     .replace('7', "\\")
            //     .replace('F', "/");
            // println!("{tmp}  {sum}");
        }

        sum
    }
}
impl Problem {
    pub fn check_up_down(&self, x: usize, y: usize) -> bool {
        let mut i = y - 1;
        let mut horizontal = 0;
        while i > 0 {
            if self.path[i][x] == '-' {
                horizontal += 1;
            } else if self.path[i][x] == 'L' {
                loop {
                    i -= 1;
                    if self.path[i][x] == '7' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == 'F' {
                        break;
                    }
                }
            } else if self.path[i][x] == 'J' {
                loop {
                    i -= 1;
                    if self.path[i][x] == 'F' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == '7' {
                        break;
                    }
                }
            }
            i -= 1;
        }
        // if y > 128 {
        //     println!("row {y}, col {x}, up:   {horizontal}");
        // }
        if horizontal == 0 {
            return false;
        }

        horizontal = 0;
        i = y + 1;
        while i < self.path.len() {
            // if y > 128 {
            //     println!("Checking {i} {x}, {}", self.path[i][x]);
            // }
            if self.path[i][x] == '-' {
                horizontal += 1;
            } else if self.path[i][x] == '7' {
                loop {
                    i += 1;
                    if self.path[i][x] == 'L' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == 'J' {
                        break;
                    }
                }
            } else if self.path[i][x] == 'F' {
                loop {
                    i += 1;
                    if self.path[i][x] == 'J' {
                        horizontal += 1;
                        break;
                    } else if self.path[i][x] == 'L' {
                        break;
                    }
                }
            }
            i += 1;
        }
        // if y > 128 {
        //     println!("row {y}, col {x}, down: {horizontal}");
        // }
        horizontal != 0
    }

    pub fn follow_path(&mut self, x: usize, y: usize) -> i64 {
        let mut steps: i64 = 1;
        let mut walk_x = x;
        let mut walk_y = y;
        let mut cur = self.grid[y][x];
        let mut last_x = self.start_col;
        let mut last_y = self.start_row;

        // println!("Start: {cur} @ {walk_y}, {walk_x}");
        // println!("{}, {}", self.start_row, self.start_col);
        self.path[last_y][last_x] = 'S';

        while !(walk_x == self.start_col && walk_y == self.start_row) {
            self.path[walk_y][walk_x] = cur;
            let tmp_x = walk_x;
            let tmp_y = walk_y;
            // print!("{cur}");
            // let old = cur;
            // Right
            if last_x != walk_x + 1 && (cur == '-' || cur == 'L' || cur == 'F') {
                let new = self.grid[walk_y][walk_x + 1];
                // println!("  Right: {new}");
                if new == '-' || new == 'J' || new == '7' || new == 'S' {
                    walk_x += 1;
                    cur = new;
                }
            }
            // Left
            if last_x != walk_x - 1
                && (walk_x == tmp_x && walk_y == tmp_y)
                && (cur == '-' || cur == 'J' || cur == '7')
            {
                let new = self.grid[walk_y][walk_x - 1];
                // println!("  Left: {new}");
                if new == '-' || new == 'L' || new == 'F' || new == 'S' {
                    walk_x -= 1;
                    cur = new;
                }
            }
            // Up
            if last_y != walk_y - 1
                && (walk_x == tmp_x && walk_y == tmp_y)
                && (cur == '|' || cur == 'J' || cur == 'L')
            {
                let new = self.grid[walk_y - 1][walk_x];
                // println!("  Up: {new}");
                if new == '|' || new == '7' || new == 'F' || new == 'S' {
                    walk_y -= 1;
                    cur = new;
                }
            }
            // Down
            if last_y != walk_y + 1
                && (walk_x == tmp_x && walk_y == tmp_y)
                && (cur == '|' || cur == '7' || cur == 'F')
            {
                let new = self.grid[walk_y + 1][walk_x];
                // println!("  Down: {new}");
                if new == '|' || new == 'J' || new == 'L' || new == 'S' {
                    walk_y += 1;
                    cur = new;
                }
            }

            // If we did not move, this path is dead
            if walk_x == tmp_x && walk_y == tmp_y {
                // println!("\n cur == old @ {walk_y}, {walk_x}");
                return 0;
            }
            if walk_x == self.start_col && walk_y == self.start_row {
                // println!("\n start");
                return steps;
            }
            last_x = tmp_x;
            last_y = tmp_y;
            steps += 1;
        }
        0
    }

    pub fn new(data: &[String]) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut start_row: usize = 0;
        let mut start_col: usize = 0;

        // 1 space padding around the grid to make out of bounds easier to handle
        let mut g: Vec<char> = vec![' '; data[0].len() + 2];
        grid.push(g);
        for (i, line) in data.iter().enumerate() {
            g = vec![' '];
            g.append(&mut line.chars().collect::<Vec<char>>());
            g.push(' ');

            grid.push(g);
            if start_row == 0 {
                if let Some(f) = line.find('S') {
                    start_row = i + 1;
                    start_col = f + 1;
                }
            }
        }
        let g: Vec<char> = vec![' '; data[0].len() + 2];
        grid.push(g);

        // for row in &grid {
        //     for c in row {
        //         print!("{}", c);
        //     }
        //     println!();
        // }
        // println!("Start: {}, {}", start_row, start_col);
        let cols = grid[0].len();
        let rows = grid.len();

        Problem {
            grid,
            start_row,
            start_col,
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
