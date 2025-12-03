pub fn process(input: Vec<String>) -> i64 {
    let mut total = 0;
    for line in input {
        let mut highest_first = -1;
        let mut highest_second = -1;
        for (i, el) in line.chars().enumerate() {
            // Make sure we're not in the last character
            let digit = el.to_digit(10).expect("cannot parse digit") as i64;

            if i < line.len() - 1 {
                if digit > highest_first {
                    highest_first = digit;
                    highest_second = -1;
                    continue;
                }
            }

            if digit > highest_second {
                highest_second = digit;
                continue;
            }
        }

        dbg!(highest_first, highest_second);

        let biggest_number = (highest_first * 10) + highest_second;

        total += biggest_number
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day03/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 357);
    }

    #[test]
    fn real() {
        let result = process(load_input("day03/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, -1);
    }
}
