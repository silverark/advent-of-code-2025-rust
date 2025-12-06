pub fn process(input: Vec<String>) -> u128 {
    let mut total: u128 = 0;
    for line in input {
        let mut start = 0;

        let mut calculated_number: u128 = 0;
        let numbers: Vec<u128> = line
            .chars()
            .map(|c| c.to_digit(10).expect("cannot parse digit") as u128)
            .collect();

        for pos in 0..12 {
            let search = 12 - pos - 1;
            let last_pos = line.len() - search - 1;
            let mut highest = 0;
            let mut found_pos = start;

            for i in start..=last_pos {
                if numbers[i] > highest {
                    highest = numbers[i];
                    found_pos = i;
                    if highest == 9 {
                        break;
                    }
                }
            }
            calculated_number = (calculated_number * 10) + highest;
            start = found_pos + 1;
        }
        total += calculated_number;
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
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn real() {
        let result = process(load_input("day03/input.txt"));
        println!("real = {}", result);
    }
}
