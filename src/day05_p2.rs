pub fn process(input: Vec<String>) -> i64 {
    let mut split_index = 0;
    for (i, line) in input.iter().enumerate() {
        if line.is_empty() {
            split_index = i;
            break;
        }
    }

    let (range_lines, _) = input.split_at(split_index);
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in range_lines {
        let (start, end) = line.split_once('-').expect("Invalid range line");
        let start: i64 = start.parse().expect("Invalid start");
        let end: i64 = end.parse().expect("Invalid end");
        ranges.push((start, end));
    }
    ranges = merge_ranges(ranges);
    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day05/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 14);
    }

    #[test]
    fn real() {
        let result = process(load_input("day05/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, 342018167474526);
    }

    #[test]
    fn merge_ranges_test() {
        let ranges = vec![(100, 101), (99, 100), (1, 3), (2, 5), (10, 12), (11, 15)];
        let merged = merge_ranges(ranges);
        assert_eq!(merged, vec![(1, 5), (10, 15), (99, 101)]);
    }
}
