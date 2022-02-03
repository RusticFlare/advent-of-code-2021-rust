use itertools::Itertools;
use std::collections::HashMap;

pub mod input;

pub fn part_1(input: &str) -> usize {
    let subs: HashMap<(char, char), char> = input
        .lines()
        .dropping(2)
        .map(|l| l.split(" -> "))
        .map(|mut s| (s.next().unwrap().chars(), s.next().unwrap().chars()))
        .map(|(mut from, mut to)| {
            (
                (from.next().unwrap(), from.next().unwrap()),
                to.next().unwrap(),
            )
        })
        .collect();

    let mut chain = String::from(input.lines().next().unwrap());

    for _ in 0..10 {
        let last = chain.chars().last().unwrap();

        chain = chain
            .chars()
            .tuple_windows::<(char, char)>()
            .flat_map(|(a, b)| [a, *subs.get(&(a, b)).unwrap()])
            .collect::<String>();

        chain.push(last);
    }

    let char_counts = chain.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    let (min, max) = char_counts.values().minmax().into_option().unwrap();

    return max - min;
}

pub fn part_2(input: &str) -> u128 {
    0
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(1588, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2188189693529, part_2(input::TEST_INPUT));
    }
}
