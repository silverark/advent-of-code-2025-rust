pub fn process(input: Vec<String>) -> i64 {
    let coords: Vec<Coord> = input.iter().map(|s| Coord::from(s.as_str())).collect();
    let length = coords.len();
    let rectangles = rectangles(&coords);

    let (p1, p2) = rectangles
        .iter()
        .find(|(p1, p2)| {
            let (xmin, xmax, ymin, ymax) = edges(p1, p2);
            for (i, c1) in coords.iter().enumerate() {
                let c2 = &coords[(i + 1) % length];

                if c1.0 == c2.0 {
                    let (ylmin, ylmax) = (c1.1.min(c2.1), c1.1.max(c2.1));
                    if xmin < c1.0 && xmax > c1.0 && !(ymin >= ylmax || ymax <= ylmin) {
                        return false;
                    }
                } else if c1.1 == c2.1 {
                    let (xlmin, xlmax) = (c1.0.min(c2.0), c1.0.max(c2.0));
                    if ymin < c1.1 && ymax > c1.1 && !(xmin >= xlmax || xmax <= xlmin) {
                        return false;
                    }
                } else {
                    panic!("Unreachable");
                }
            }
            true
        })
        .unwrap();

    p1.area(p2) as i64
}

fn edges(point1: &Coord, point2: &Coord) -> (u64, u64, u64, u64) {
    (
        point1.0.min(point2.0),
        point1.0.max(point2.0),
        point1.1.min(point2.1),
        point1.1.max(point2.1),
    )
}

fn rectangles(coords: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut pairs = vec![];
    for i in 0..(coords.len() - 1) {
        for j in i + 1..coords.len() {
            pairs.push((coords[i].clone(), coords[j].clone()))
        }
    }

    pairs.sort_by(|p1, p2| p1.0.area(&p1.1).cmp(&p2.0.area(&p2.1)));
    pairs.reverse();

    for pair in &pairs {
        println!("{}", pair.0.area(&pair.1));
    }

    pairs
}

#[derive(Clone)]
struct Coord(u64, u64);

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let nums: Vec<u64> = value.split(",").map(|n| n.parse().unwrap()).collect();
        match nums.as_slice() {
            [x, y] => Self(*x, *y),
            _ => panic!("Expected 2-element vector"),
        }
    }
}

impl Coord {
    fn area(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
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
    }
}
