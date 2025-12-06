pub fn process(input: Vec<String>) -> i64 {
    let mut split_index = 0;
    for (i, line) in input.iter().enumerate() {
        if line.is_empty() {
            split_index = i;
            break;
        }
    }

    let (range_lines, rest) = input.split_at(split_index);
    let id_lines = &rest[1..];

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in range_lines {
        let (start, end) = line.split_once('-').expect("Invalid range line");
        let start: i64 = start.parse().expect("Invalid start");
        let end: i64 = end.parse().expect("Invalid end");
        ranges.push((start, end));
    }

    let mut ids: Vec<i64> = Vec::new();
    for line in id_lines {
        let id: i64 = line.parse().expect("Invalid ingredient");
        ids.push(id);
    }

    let mut ingredient_count = 0;
    for id in &ids {
        assert!(*id >= 0, "ID cannot be negative: {}", id);
        if in_ranges(*id, &ranges) {
            ingredient_count += 1;
        }
    }

    ingredient_count
}

fn in_ranges(id: i64, ranges: &[(i64, i64)]) -> bool {
    for &(start, end) in ranges {
        if id >= start && id <= end {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day05/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn real() {
        let result = process(load_input("day05/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, 643);
    }
}
