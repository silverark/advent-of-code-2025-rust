pub fn process(input: Vec<String>) -> u64 {
    let nums: Vec<(i64, i64)> = input
        .iter()
        .map(|line| {
            let mut parts = line.trim().split(',');
            let x = parts.next().expect("missing x").parse::<i64>().expect("invalid x");
            let y = parts.next().expect("missing y").parse::<i64>().expect("invalid y");
            (x, y)
        })
        .collect();
    max_area(nums)
}

fn max_area(nums: Vec<(i64, i64)>) -> u64 {
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for i in 0..(nums.len() - 1) {
        for j in (i + 1)..nums.len() {
            rectangles.push(Rectangle::from((nums[i], nums[j])));
        }
    }
    // Order by desc area
    rectangles.sort_by(|a, b| b.area().cmp(&a.area()));

    for rect in &rectangles {
        if rect.wall_intercepts(&nums) == false {
            return rect.area();
        }
    }
    0
}

struct Rectangle {
    p1: (i64, i64),
    p2: (i64, i64),
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}
impl From<((i64, i64), (i64, i64))> for Rectangle {
    fn from((p1, p2): ((i64, i64), (i64, i64))) -> Self {
        let x_min = p1.0.min(p2.0);
        let x_max = p1.0.max(p2.0);
        let y_min = p1.1.min(p2.1);
        let y_max = p1.1.max(p2.1);
        Rectangle {
            p1,
            p2,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}
impl Rectangle {
    fn area(&self) -> u64 {
        (self.p1.0.abs_diff(self.p2.0) + 1) * (self.p1.1.abs_diff(self.p2.1) + 1)
    }
    fn wall_intercepts(&self, nums: &[(i64, i64)]) -> bool {
        for (i, num1) in nums.iter().enumerate() {
            let num2 = &nums[(i + 1) % nums.len()];
            if num1.0 == num2.0 {
                let (ylmin, ylmax) = (num1.1.min(num2.1), num1.1.max(num2.1));
                if self.x_min < num1.0 && self.x_max > num1.0 && !(self.y_min >= ylmax || self.y_max <= ylmin) {
                    return true;
                }
            } else if num1.1 == num2.1 {
                let (xlmin, xlmax) = (num1.0.min(num2.0), num1.0.max(num2.0));
                if self.y_min < num1.1 && self.y_max > num1.1 && !(self.x_min >= xlmax || self.x_max <= xlmin) {
                    return true;
                }
            } else {
                panic!("wtf");
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day09/sample.txt"));
        println!("sample = {}", result);
        assert_eq!(result, 24);
    }

    #[test]
    fn real() {
        let result = process(load_input("day09/input.txt"));
        println!("real = {}", result);
        assert_eq!(result, 1476550548);
    }
}
