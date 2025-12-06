pub fn process(input: Vec<String>) -> i64 {
    let mut grid: Vec<Vec<u8>> = input.iter().map(|line| line.as_bytes().to_vec()).clone().collect();
    let mut total_removed = 0;
    let mut removed = -1;
    while removed != 0 {
        removed = remove_rolls(&mut grid);
        total_removed += removed;
    }
    return total_removed;
}

fn remove_rolls(grid: &mut [Vec<u8>]) -> i64 {
    let height = grid.len();
    let width = grid[0].len();
    let directions: &[(i64, i64)] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut rolls_to_remove: Vec<(usize, usize)> = Vec::new();

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut neighbors = 0;

            for (dy, dx) in directions {
                let ny = y as i64 + dy;
                let nx = x as i64 + dx;

                if ny < 0 || ny >= height as i64 || nx < 0 || nx >= width as i64 {
                    continue;
                }

                if grid[ny as usize][nx as usize] == b'@' {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
                count += 1;
                // Mark the roll for deletion
                rolls_to_remove.push((y, x));
            }
        }
    }

    // remove the rolls from the grid
    for (y, x) in rolls_to_remove {
        grid[y][x] = b'.';
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day04/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 43);
    }

    #[test]
    fn real() {
        let result = process(load_input("day04/input.txt"));
        println!("real = {}", result);
    }
}
