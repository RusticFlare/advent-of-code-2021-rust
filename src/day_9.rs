pub mod input;

pub fn part_1(input: &str) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    map.iter()
        .enumerate()
        .map(|(r, row)| {
            let up = r.checked_sub(1);
            let same = &r;
            let down = r.checked_add(1);
            row.into_iter()
                .enumerate()
                .filter(|(c, v)| {
                    up.and_then(|x| map.get(x))
                        .and_then(|x| x.get(*c))
                        .unwrap_or(&10)
                        > v
                        && down
                            .and_then(|x| map.get(x))
                            .and_then(|x| x.get(*c))
                            .unwrap_or(&10)
                            > v
                        && c.checked_sub(1)
                            .and_then(|left| map[*same].get(left))
                            .unwrap_or(&10)
                            > v
                        && c.checked_add(1)
                            .and_then(|right| map[*same].get(right))
                            .unwrap_or(&10)
                            > v
                })
                .map(|(_, v)| v + 1)
                .collect::<Vec<u32>>()
        })
        .flatten()
        .sum()
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
        assert_eq!(15, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1134, part_2(input::TEST_INPUT));
    }
}
