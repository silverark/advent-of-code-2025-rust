pub fn process(input: Vec<String>) -> i64 {
    let mut rows: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    if let Some((last, rest)) = input.split_last() {
        for line in rest {
            let numbers: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            rows.push(numbers);
        }
        operators = last.split_whitespace().collect();
    }

    let col_length = rows[0].len();
    let mut total_sum = 0;
    for i in 0..col_length {
        let operator = operators[i];
        let mut calc_sum = rows[0][i];
        for x in 1..rows.len() {
            let current_value = rows[x][i];
            match operator {
                "+" => calc_sum += current_value,
                "-" => calc_sum -= current_value,
                "*" => calc_sum *= current_value,
                "/" => calc_sum /= current_value,
                _ => {
                    panic!("Unknown operator: {}", operator);
                }
            }
        }

        total_sum += calc_sum;
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day06/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn real() {
        let result = process(load_input("day06/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, 4722948564882)
    }
}
