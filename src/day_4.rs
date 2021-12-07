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
                .map(|row| Box::new(row.iter().map(|&b| b)) as Box<dyn Iterator<Item = bool>>)
                .reduce(|acc, row| {
                    Box::new(acc.zip(row).map(|(a, b)| a & b)) as Box<dyn Iterator<Item = bool>>
                })
                .unwrap()
                .any(|b| b)
    }

    fn score(&self) -> u32 {
        let vs = self.values.iter().flat_map(|row| row.iter());
        let ms = self.marked.iter().flat_map(|row| row.iter());
        return vs
            .zip(ms)
            .filter_map(|(v, &m)| if m { Option::None } else { Option::Some(v) })
            .sum();
    }

    fn mark_value(&self, value: u32) -> BingoCard {
        BingoCard {
            values: self.values.to_vec(),
            marked: self
                .marked
                .iter()
                .enumerate()
                .map(|(r, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(c, &old)| {
                            if self.values[r][c] == value {
                                true
                            } else {
                                old
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut bingo_cards = bingo_cards(input);

    let values = values_called(input);

    for value in values {
        bingo_cards = bingo_cards
            .iter()
            .map(|card| card.mark_value(value))
            .collect();
        let winner = bingo_cards.iter().find(|card| card.is_winner());
        if winner.is_some() {
            return winner.unwrap().score() * value;
        }
    }
    return 0;
}

fn values_called(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}

fn bingo_cards(input: &str) -> Vec<BingoCard> {
    input
        .split("\n\n")
        .dropping(1)
        .map(|s| {
            s.lines()
                .map(|l| {
                    l.trim()
                        .split_whitespace()
                        .map(|i| i.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .map(|card_values| BingoCard::new(card_values))
        .collect()
}

pub fn part_2(input: &str) -> u32 {
    let mut bingo_cards = bingo_cards(input);

    let values = values_called(input);

    for value in values {
        let score = if bingo_cards.len() == 1 {
            Option::Some(bingo_cards[0].score())
        } else {
            Option::None
        };
        bingo_cards = bingo_cards
            .iter()
            .map(|card| card.mark_value(value))
            .filter(|card| !card.is_winner())
            .collect();
        if bingo_cards.len() == 0 {
            return (score.unwrap() - value) * value;
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
        assert_eq!(4512, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1924, part_2(input::TEST_INPUT));
    }
}
