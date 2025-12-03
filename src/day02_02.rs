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

    for chunk_len in 1..=number_str_length / 2 {
        if number_str_length % chunk_len != 0 {
            continue;
        }

        let repeats = number_str_length / chunk_len;
        if repeats < 2 {
            continue;
        }

        let chunk = &numer_str[0..chunk_len];
        let mut ok = true;

        for i in 1..repeats {
            let start = i * chunk_len;
            let end = start + chunk_len;
            if &numer_str[start..end] != chunk {
                ok = false;
                break;
            }
        }

        if ok {
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
        let result = process(load_input("day02/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn real() {
        let result = process(load_input("day02/input.txt"));
        println!("real = {}", result);
    }
}
