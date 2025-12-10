pub fn process(input: Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        // Lines are 2,3,2
        let result = process(load_input("dayx/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 7);
    }

    #[test]
    fn real() {
        let result = process(load_input("dayx/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, -1);
    }
}
