pub fn process(lines: Vec<String>) -> i64 {
    let mut dial = 50;
    let mut zero_count = 0;

    for instruction in lines {
        let direction = &instruction[0..1];
        let amount = instruction[1..instruction.len()].parse::<i64>().expect("parse error");
        let (new_dial, trigger_zero) = move_dial(dial, amount, direction);

        dial = new_dial;
        zero_count += trigger_zero;
    }

    zero_count
}

fn move_dial(dial: i64, amount: i64, direction: &str) -> (i64, i64) {
    let turns = amount % 100;
    let mut zero_count = amount / 100;
    if direction == "R" {
        let new_pos = (dial + turns) % 100;
        if new_pos < dial {
            zero_count += 1;
        }
        (new_pos, zero_count)
    } else {
        let new_pos = (dial - turns + 100) % 100;
        if (new_pos > dial || new_pos == 0) && dial != 0 {
            zero_count += 1;
        }
        (new_pos, zero_count)
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
        assert_eq!(result, 6);
    }

    #[test]
    fn real() {
        let result = process(load_input("day01/input.txt"));
        println!("day01 part1 answer = {}", result);
    }

    #[test]
    fn move_dial_test() {
        assert_eq!(move_dial(99, 1, "R"), (0, 1));
        assert_eq!(move_dial(0, 1, "L"), (99, 0));
        assert_eq!(move_dial(90, 10, "R"), (0, 1));
        assert_eq!(move_dial(90, 100, "R"), (90, 1));
        assert_eq!(move_dial(90, 100, "L"), (90, 1));
        assert_eq!(move_dial(9, 10, "L"), (99, 1));
        assert_eq!(move_dial(9, 200, "L"), (9, 2));
        assert_eq!(move_dial(0, 200, "L"), (0, 2));
        assert_eq!(move_dial(0, 1, "R"), (1, 0));
        assert_eq!(move_dial(55, 55, "L"), (0, 1));
    }
}
