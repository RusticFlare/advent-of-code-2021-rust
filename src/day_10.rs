use itertools::Itertools;

pub mod input;

pub fn part_1(input: &str) -> u32 {
    input.lines().map(|line| get_score(line)).sum()
}

fn get_score(input: &str) -> u32 {
    let mut vec = Vec::new();
    let mut score = 0;
    input.chars().for_each(|c| {
        if is_open(c) {
            vec.push(c)
        } else {
            if !(matches(vec.pop().unwrap(), c)) {
                if score == 0 {
                    score = get_points(c)
                }
            }
        }
    });
    return score;
}

fn matches(open: char, close: char) -> bool {
    open == '(' && close == ')'
        || open == '[' && close == ']'
        || open == '{' && close == '}'
        || open == '<' && close == '>'
}

fn get_points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

pub fn part_2(input: &str) -> u64 {
    let scores: Vec<u64> = input
        .lines()
        .map(|line| get_autocomplete_score(line))
        .filter(|&score| score != 0)
        .sorted()
        .collect();
    return scores[scores.len() / 2];
}

fn get_autocomplete_score(input: &str) -> u64 {
    let mut vec = Vec::new();
    for c in input.chars() {
        if is_open(c) {
            vec.push(c)
        } else {
            if !(matches(vec.pop().unwrap(), c)) {
                return 0;
            }
        }
    }
    return vec
        .iter()
        .rev()
        .map(|&c| get_autocomplete_points(c))
        .reduce(|acc, points| (acc * 5) + points)
        .unwrap();
}

fn get_autocomplete_points(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(26397, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(288957, part_2(input::TEST_INPUT));
    }
}
