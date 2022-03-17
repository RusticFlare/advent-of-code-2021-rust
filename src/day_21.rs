use itertools::{process_results, Itertools};
use std::collections::HashMap;

struct Dice {
    next: u32,
    count: u32,
}

impl Dice {
    fn get_next(&mut self) -> u32 {
        let result = self.next + 1;
        self.next = result % 100;
        self.count += 1;
        result
    }
}

pub fn part_1(player_1_start: u32, player_2_start: u32) -> u32 {
    let mut player_1_pos: u32 = player_1_start - 1;
    let mut player_2_pos: u32 = player_2_start - 1;
    let mut player_1_score: u32 = 0;
    let mut player_2_score: u32 = 0;
    let mut player_1_turn: bool = true;
    let mut dice = Dice { next: 0, count: 0 };
    loop {
        if player_1_turn {
            player_1_pos =
                (player_1_pos + dice.get_next() + dice.get_next() + dice.get_next()) % 10;
            player_1_score += player_1_pos + 1;
            if player_1_score >= 1000 {
                return player_2_score * dice.count;
            }
        } else {
            player_2_pos =
                (player_2_pos + dice.get_next() + dice.get_next() + dice.get_next()) % 10;
            player_2_score += player_2_pos + 1;
            if player_2_score >= 1000 {
                return player_1_score * dice.count;
            }
        }
        player_1_turn = !player_1_turn;
    }
}

pub fn part_2(player_1: u128, player_2: u128) -> u128 {
    let player_1_finishers_per_round = get_finishers(player_1);
    let player_2_finishers_per_round = get_finishers(player_2);
    // let player_1_wins =
    player_1_finishers_per_round
        .iter()
        .enumerate()
        .map(|(round, (one_count, _))| {
            let (f, s) = player_2_finishers_per_round[round];
            (f + s) * one_count
        })
        .sum() //;
               // let total_games = 341960390180808 + 444356092776315;
               // let player_2_wins = total_games - player_1_wins;
               // if player_1_wins > player_2_wins {
               //     player_1_wins
               // } else {
               //     player_2_wins
               // }
}

fn get_finishers(start: u128) -> Vec<(u128, u128)> {
    let mut player_state: HashMap<(u128, u128), u128> = HashMap::new();
    player_state.insert((start - 1, 0 as u128), 1 as u128);
    let mut finishers_per_round = Vec::new();
    while !player_state.is_empty() {
        let mut map: HashMap<(u128, u128), u128> = HashMap::new();
        let mut finishers: u128 = 0;
        let mut survivors: u128 = 0;
        player_state
            .iter()
            .flat_map(|((pos, score), count)| {
                [
                    next(pos, score, count, 3, 1),
                    next(pos, score, count, 4, 3),
                    next(pos, score, count, 5, 6),
                    next(pos, score, count, 6, 7),
                    next(pos, score, count, 7, 6),
                    next(pos, score, count, 8, 3),
                    next(pos, score, count, 9, 1),
                ]
                .into_iter()
            })
            .flat_map(|((pos, score), count)| {
                if score >= 21 {
                    finishers += count;
                    None
                } else {
                    survivors += count;
                    Some(((pos, score), count))
                }
            })
            .for_each(|(x, count)| {
                let c = map.entry(x).or_insert_with(|| 0);
                *c += count;
            });
        player_state = map;
        finishers_per_round.push((finishers, survivors));
    }
    finishers_per_round
}

fn next(
    pos: &u128,
    score: &u128,
    count: &u128,
    shift: u128,
    count_mul: u128,
) -> ((u128, u128), u128) {
    let new_pos = (pos + shift) % 10;
    let new_score = score + new_pos + 1;
    let new_count = count * count_mul;
    ((new_pos, new_score), new_count)
}

#[cfg(test)]
mod test {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(739785, part_1(4, 8));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(444356092776315, part_2(4, 8));
    }
}
