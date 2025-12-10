pub fn process(input: Vec<String>) -> i64 {
    let nums: Vec<(i64, i64)> = input
        .iter()
        .map(|line| {
            let mut parts = line.trim().split(',');
            let x = parts.next().expect("missing x").parse::<i64>().expect("invalid x");
            let y = parts.next().expect("missing y").parse::<i64>().expect("invalid y");
            (x, y)
        })
        .collect();
    max_area(nums)
}

fn max_area(nums: Vec<(i64, i64)>) -> i64 {
    let mut max_area = 0;
    for i in 0..nums.len() {
        let (x1, y1) = nums[i];
        for j in (i + 1)..nums.len() {
            let (x2, y2) = nums[j];
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day09/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 50);
    }

    #[test]
    fn real() {
        let result = process(load_input("day09/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, 4776100539);
    }
}
