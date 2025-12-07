use std::collections::HashMap;

pub fn process(input: Vec<String>) -> i64 {
    let grid: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    // get start column of S
    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap_or(0);

    // Adding this in after brute force attempt didn't finish in 15 minutes. I think it would take weeks.
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();

    travel_paths(0, start_col as i64, &grid, &mut cache)
}

fn travel_paths(row: i64, col: i64, grid: &Vec<Vec<char>>, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    let cols = grid[0].len() as i64;
    let rows = grid.len() as i64;

    // exit left or right
    if col < 0 || col >= cols {
        return 1;
    }

    //exit bottom
    if row >= rows {
        return 1;
    }

    if let Some(&val) = cache.get(&(row, col)) {
        return val;
    }

    let mut moving_row = row;

    loop {
        // Reached bottom
        if moving_row >= rows {
            cache.insert((row, col), 1);
            return 1;
        }

        if grid[moving_row as usize][col as usize] == '^' {
            // Split paths
            let left_paths = travel_paths(moving_row + 1, col - 1, grid, cache);
            let right_paths = travel_paths(moving_row + 1, col + 1, grid, cache);
            let path_total = left_paths + right_paths;
            cache.insert((row, col), path_total);
            return path_total;
        }

        // probably just a . now so keep going down
        moving_row += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day07/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 40);
    }

    #[test]
    fn real() {
        let result = process(load_input("day07/input.txt"));
        println!("real = {}", result);
    }
}
