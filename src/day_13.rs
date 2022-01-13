use itertools::Itertools;

pub mod input;

pub fn part_1(input: &str) -> usize {
    let mut fold = input
        .split("\n\n")
        .dropping(1)
        .next()
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .dropping(2)
        .next()
        .unwrap()
        .split("=");
    let fold_direction = fold.next().unwrap();
    let fold_value = fold.next().unwrap().parse::<usize>().unwrap();

    input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|n| n.parse::<usize>().unwrap()))
        .map(|mut c| (c.next().unwrap(), c.next().unwrap()))
        .map(|(x, y)| {
            if fold_direction == "y" && y > fold_value {
                (x, fold_value - (y - fold_value))
            } else if fold_direction == "x" && x > fold_value {
                (fold_value - (x - fold_value), y)
            } else {
                (x, y)
            }
        })
        .unique()
        .count()
}

pub fn part_2(input: &str) -> usize {
    let mut paper: Vec<(usize, usize)> = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|n| n.parse::<usize>().unwrap()))
        .map(|mut c| (c.next().unwrap(), c.next().unwrap()))
        .collect();
    for (fold_direction, fold_value) in input
        .split("\n\n")
        .dropping(1)
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split_whitespace().dropping(2).next().unwrap().split("="))
        .map(|mut fold| {
            (
                fold.next().unwrap(),
                fold.next().unwrap().parse::<usize>().unwrap(),
            )
        })
    {
        paper = paper
            .iter()
            .map(|(x, y)| {
                if fold_direction == "y" && y > &fold_value {
                    (*x, fold_value - (y - fold_value))
                } else if fold_direction == "x" && x > &fold_value {
                    (fold_value - (x - fold_value), *y)
                } else {
                    (*x, *y)
                }
            })
            .collect()
    }
    for y in (0 as usize)..*paper.iter().map(|(x, y)| y).max().unwrap() + 1 {
        println!();
        for x in (0 as usize)..*paper.iter().map(|(x, y)| x).max().unwrap() + 1 {
            print!("{}", if paper.contains(&(x, y)) { "#" } else { "." })
        }
    }
    return 0;
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(17, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(input::TEST_INPUT));
    }
}
