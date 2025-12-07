pub fn process(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let mut total_splits = 0;

    let cols = input[0].len();

    for row in 1..(input.len() - 1) {
        for col in 0..cols - 1 {
            if grid[row][col] == '.' {
                // See if the Above is a line or an S and extend
                if grid[row - 1][col] == '|' || grid[row - 1][col] == 'S' {
                    grid[row][col] = '|';
                }
            }

            if grid[row][col] == '^' {
                // CHeck row above has a line going to it, and if so add line to left and right
                if grid[row - 1][col] == '|' {
                    // Splitting so increase count
                    total_splits += 1;

                    if col > 0 {
                        grid[row][col - 1] = '|';
                    }
                    if col < cols - 1 {
                        grid[row][col + 1] = '|';
                    }
                }
            }
        }
    }

    // Print out the grid
    // for row in &grid {
    //     let line: String = row.iter().collect();
    //     println!("{}", line);
    // }

    total_splits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day07/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 21);
    }

    #[test]
    fn real() {
        let result = process(load_input("day07/input.txt"));
        println!("real = {}", result);
    }
}
