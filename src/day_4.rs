use itertools::Itertools;

pub mod input;

struct BingoCard {
    values: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
}

impl BingoCard {
    fn new(values: Vec<Vec<u32>>) -> Self {
        Self {
            marked: vec![vec![false; values[0].len()]; values.len()],
            values,
        }
    }

    fn is_winner(&self) -> bool {
        self.marked.iter().any(|row| row.iter().all(|&b| b))
            || self
                .marked
                .iter()
                .map(|row| row.iter().map(|&b| b).collect::<Vec<bool>>())
                .reduce(|acc, row| acc.iter().zip(row.iter()).map(|(a, b)| a & b).collect())
                .unwrap()
                .iter()
                .any(|&b| b)
    }

    fn score(&self) -> u32 {
        self.values
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, &value)| if self.marked[r][c] { 0 } else { value })
                    .sum::<u32>()
            })
            .sum()
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut bingo_cards = input
        .split("\n\n")
        .dropping(1)
        .map(|s| s.lines())
        .map(|lines| {
            lines.map(|l| {
                l.trim()
                    .split_whitespace()
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect()
            })
        })
        .map(|s| s.collect::<Vec<Vec<u32>>>())
        .map(|card_values| BingoCard::new(card_values))
        .collect::<Vec<BingoCard>>();

    let values = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap());

    for value in values {
        bingo_cards = bingo_cards
            .iter()
            .map(|card| BingoCard {
                values: card.values.iter().map(|i| i.iter().map(|&j|j).collect()).collect(),
                marked: card
                    .marked
                    .iter()
                    .enumerate()
                    .map(|(r, row)| {
                        row.iter()
                            .enumerate()
                            .map(|(c, &old)| {
                                if card.values[r][c] == value {
                                    true
                                } else {
                                    old
                                }
                            })
                            .collect()
                    })
                    .collect(),
            })
            .collect();
        let winner = bingo_cards.iter().find(|card| card.is_winner());
        if winner.is_some() {
            return winner.unwrap().score() * value;
        }
    }
    return 0;
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
        assert_eq!(4512, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(230, part_2(input::TEST_INPUT));
    }
}
