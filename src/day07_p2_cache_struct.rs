use std::collections::HashMap;
use std::hash::Hash;

// Generic cache method. Same as day07_p2 but can be used again.
struct Cache<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    fn new() -> Self {
        Cache { map: HashMap::new() }
    }

    fn call<F>(&mut self, key: K, f: F) -> V
    where
        F: FnOnce(&mut Self) -> V,
    {
        // Check if we have the value already
        if self.map.contains_key(&key) {
            return self.map.get(&key).unwrap().clone();
        }
        // COmpute the value and strore it in the cache
        let value = f(self);
        self.map.insert(key, value.clone());
        value
    }
}

pub fn process(input: Vec<String>) -> i64 {
    let grid: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    // get start column of S
    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap_or(0);

    // Adding this in after brute force attempt didn't finish in 15 minutes.
    let mut cache: Cache<(i64, i64), i64> = Cache::new();

    travel_paths(0, start_col as i64, &grid, &mut cache)
}

fn travel_paths(row: i64, col: i64, grid: &Vec<Vec<char>>, cache: &mut Cache<(i64, i64), i64>) -> i64 {
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

    cache.call((row, col), |_cache| {
        let mut moving_row = row;

        loop {
            // Reached bottom
            if moving_row >= rows {
                return 1;
            }

            if grid[moving_row as usize][col as usize] == '^' {
                // Split paths - we need to compute these recursively
                // But we can't do it inside this closure due to borrowing
                // So we return a sentinel value and handle it outside
                // Actually, let's restructure this differently...

                // The issue is we can't call travel_paths from within the closure
                // because it needs mutable access to cache.
                // We need to compute the value without recursive calls in the closure
                return -moving_row - 1; // Return negative to indicate split needed
            }

            // probably just a . now so keep going down
            moving_row += 1;
        }
    });

    // Now check if we got a split indicator
    let cached_value = *cache.map.get(&(row, col)).unwrap();
    if cached_value < 0 {
        // This indicates a split at row = -cached_value - 1
        let split_row = -cached_value - 1;
        let left_paths = travel_paths(split_row + 1, col - 1, grid, cache);
        let right_paths = travel_paths(split_row + 1, col + 1, grid, cache);
        let total = left_paths + right_paths;
        cache.map.insert((row, col), total);
        return total;
    }

    cached_value
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
