use std::collections::HashMap;

pub fn process(lines: Vec<String>, pairs_to_connect: usize) -> i64 {
    let mut points = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().trim().parse().unwrap();
        let y: i64 = parts.next().unwrap().trim().parse().unwrap();
        let z: i64 = parts.next().unwrap().trim().parse().unwrap();
        points.push(Point3D { x, y, z });
    }

    let total_points = points.len();

    #[derive(Clone, Copy)]
    struct Connection {
        dist: u128,
        a: usize,
        b: usize,
    }

    // work out all distances. Using the squared version as it's faster and just for comparison
    let mut connections = Vec::new();
    for i in 0..total_points {
        for j in (i + 1)..total_points {
            let dist2 = points[i].squared_distance_to(&points[j]) as u128;
            connections.push(Connection {
                dist: dist2,
                a: i,
                b: j,
            });
        }
    }

    connections.sort_by(|e1, e2| e1.dist.cmp(&e2.dist));

    // each point is it's own circuit to start
    let mut circuit_ids: Vec<usize> = (0..total_points).collect();

    let limit = pairs_to_connect.min(connections.len());
    for i in 0..limit {
        let e = connections[i];
        merge_circuits(&mut circuit_ids, e.a, e.b);
    }

    let mut counts: HashMap<usize, i64> = HashMap::new();
    for &circuit_id in &circuit_ids {
        *counts.entry(circuit_id).or_insert(0) += 1;
    }

    let mut sizes: Vec<i64> = counts.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending

    // Times the three largest
    sizes[0] * sizes[1] * sizes[2]
}

fn merge_circuits(circuit_id: &mut [usize], a: usize, b: usize) {
    let ca = circuit_id[a];
    let cb = circuit_id[b];
    // check if htey are in same circuit
    if ca == cb {
        return;
    }

    // keep the smallestr id. Probably faster to keep the circuit with fewer changes needed, but I don't store that!
    let keep = ca.min(cb);
    let replace = ca.max(cb);

    for id in circuit_id.iter_mut() {
        if *id == replace {
            *id = keep;
        }
    }
}

// Following code for Euclidean distance in 3D space from https://share.google/aimode/nydsPQsbjQ6kvSSK2
#[derive(Debug, Clone, Copy)] // Add Copy if you'll pass points around often
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}
impl Point3D {
    // faster than Euclidean distance and just used for comparing
    fn squared_distance_to(&self, other: &Point3D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::load_input;

    #[test]
    fn sample() {
        let result = process(load_input("day08/sample.txt"), 10);
        println!("sample = {}", result);
        assert_eq!(result, 40);
    }

    #[test]
    fn real() {
        let result = process(load_input("day08/input.txt"), 1000);
        println!("real = {}", result);
        assert_eq!(result, 103488);
    }
}
