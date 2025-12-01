pub fn process(_: Vec<String>) -> i64 {
   0
}

#[cfg(test)]
mod tests {
    use crate::util::load_input;
    use super::*;

    #[test]
    fn sample() {
        let result = process(load_input("dayx/sample.txt"));
        println!("sample = {}",result);
        assert_eq!(result, -1);
    }

    #[test]
    fn real() {
        let result = process(load_input("dayx/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, -1);
    }
}
