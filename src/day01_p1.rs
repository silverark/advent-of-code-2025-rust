pub fn process(lines: Vec<String>) -> i64 {
    let mut dial = 50;
    let mut zero_count = 0;

    for instruction in lines {
        let direction = &instruction[0..1];
        let amount = instruction[1..instruction.len()].parse::<i64>().expect("parse error");

        dial = move_dial(dial, amount, direction);
        if dial == 0 {
            zero_count += 1
        }
    }
    zero_count
}

fn move_dial(dial: i64, amount: i64, direction: &str) -> i64 {
    let turns = amount % 100;
    if direction == "R" {
        (dial + turns) % 100
    } else {
        (dial - turns + 100) % 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day01/sample.txt"));
        println!("day01 part1 sample = {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn real() {
        let result = process(load_input("day01/input.txt"));
        println!("day01 part1 answer = {}", result);
    }

    #[test]
    fn move_dial_test() {
        assert_eq!(move_dial(99, 1, "R"), 0);
        assert_eq!(move_dial(0, 1, "L"), 99);
        assert_eq!(move_dial(90, 10, "R"), 0);
        assert_eq!(move_dial(9, 10, "L"), 99);
        assert_eq!(move_dial(90, 100, "R"), 90);
        assert_eq!(move_dial(90, 100, "L"), 90);
    }
}
