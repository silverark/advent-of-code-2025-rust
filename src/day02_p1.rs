pub fn process(input: Vec<String>) -> i64 {
    let mut bad_id_total = 0;

    for range in input[0].split(",") {
        let parts = range.split("-").collect::<Vec<&str>>();

        let start = parts[0].parse::<i64>().expect("cannot parse start");
        let end = parts[1].parse::<i64>().expect("cannot parse start");

        for id in start..=end {
            if number_repeats(id) {
                bad_id_total += id;
            }
        }
    }

    bad_id_total
}

fn number_repeats(number: i64) -> bool {
    let numer_str = number.to_string();
    let number_str_length = numer_str.len();

    if number_str_length % 2 != 0 {
        return false;
    }

    let first = &numer_str[0..number_str_length / 2];
    let second = &numer_str[number_str_length / 2..number_str_length];

    first == second
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day02/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn real() {
        let result = process(load_input("day02/input.txt"));
        println!("real = {}", result);
    }
}
