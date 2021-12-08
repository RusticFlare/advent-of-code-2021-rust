use itertools::Itertools;

pub mod input;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split(" | ")
                .dropping(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.len())
        })
        .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
        .count()
}

pub fn part_2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(26, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(61229, part_2(input::TEST_INPUT));
    }
}
