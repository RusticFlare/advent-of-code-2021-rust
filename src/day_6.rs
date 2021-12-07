use std::collections::HashMap;

pub mod input;

pub fn part_1(input: &str) -> u128 {
    result(input, 80)
}

pub fn part_2(input: &str) -> u128 {
    result(input, 256)
}

fn result(input: &str, days: i32) -> u128 {
    let mut timers = input.split(",").map(|t| t.parse::<u8>().unwrap()).fold(
        HashMap::new(),
        |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        },
    );

    for _ in 0..days {
        timers = timers
            .into_iter()
            .fold(HashMap::new(), |mut acc, (t, count)| {
                if t == 0 {
                    *acc.entry(6).or_insert(0) += count;
                    *acc.entry(8).or_insert(0) += count;
                } else {
                    *acc.entry(t - 1).or_insert(0) += count;
                }
                acc
            })
    }
    return timers.values().sum();
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(5934, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(26984457539, part_2(input::TEST_INPUT));
    }
}
