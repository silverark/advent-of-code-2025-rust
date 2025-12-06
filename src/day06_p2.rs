pub fn process(input: Vec<String>) -> i64 {
    let col_length = input[0].len();
    let mut total_sum = 0;
    let mut numbers: Vec<i64> = Vec::new();
    for i in (0..col_length).rev() {
        let mut s = String::new();
        for x in 0..input.len() {
            let current_value = input[x].chars().nth(i).unwrap();
            if current_value.is_ascii_digit() {
                s.push(current_value);
                continue;
            }
            if x == input.len() - 1 {
                // Probably a blank column
                if s.is_empty() {
                    s.clear();
                    numbers.clear();
                    break;
                }

                numbers.push(s.parse().unwrap());
                s.clear();

                // If it's a blank, move on to next number.
                if current_value == ' ' {
                    break;
                }

                let mut calc_sum = numbers[0];
                // Must be an operator
                for num in &numbers[1..] {
                    match current_value as u8 as char {
                        '+' => calc_sum += *num,
                        '-' => calc_sum -= *num,
                        '*' => calc_sum *= *num,
                        '/' => calc_sum /= *num,
                        _ => {
                            panic!("Unknown operator: {}", current_value as u8 as char);
                        }
                    }
                }
                total_sum += calc_sum;
                numbers.clear();
            }
        }
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
        assert_eq!(result, 3263827);
    }

    #[test]
    fn real() {
        let result = process(load_input("day06/input.txt"));
        println!("real = {}", result);
    }
}
