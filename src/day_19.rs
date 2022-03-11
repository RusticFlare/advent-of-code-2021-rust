use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Sub;

pub mod input;

#[derive(PartialEq, Eq, Clone)]
struct Vec3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Vec3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn vec3d(input: &str) -> Vec3d {
    let vec: Vec<i32> = input.split(",").map(|v| v.parse().unwrap()).collect();
    Vec3d {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}

pub fn part_1(input: &str) -> usize {
    let x: Vec<Vec<Vec3d>> = input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .dropping(1)
                .map(|beacon| vec3d(beacon))
                .collect()
        })
        .collect();
    let mut beacons = HashSet::new();
    for beacon in x.first().unwrap() {
        beacons.insert(beacon);
    }
    x.0
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(79, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(input::TEST_INPUT));
    }
}
