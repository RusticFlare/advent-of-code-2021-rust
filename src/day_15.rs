use std::cmp;

pub mod input;

pub fn part_1(input: &str) -> u32 {
    let costs: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut total_risk = vec![vec![u32::MAX; costs[0].len()]; costs.len()];
    total_risk[0][0] = 0;

    loop {
        let old: Vec<Vec<u32>> = total_risk
            .iter()
            .map(|r| r.iter().map(|&i| i).collect())
            .collect();

        for r in 0..total_risk.len() {
            for c in 0..(total_risk[r].len()) {
                let above: Option<u32> = r
                    .checked_sub(1)
                    .and_then(|up| total_risk.get(up).map(|row| row[c]));
                let below: Option<u32> = total_risk.get(r + 1).map(|row| row[c]);
                let left: Option<u32> = c
                    .checked_sub(1)
                    .and_then(|across| total_risk.get(r).and_then(|row| row.get(across)))
                    .map(|&x| x);
                let right: Option<u32> =
                    total_risk.get(r).and_then(|row| row.get(c + 1)).map(|&x| x);
                let new = [above, below, left, right]
                    .iter()
                    .flat_map(|x| x.iter())
                    .min()
                    .unwrap()
                    .checked_add(costs[r][c])
                    .unwrap_or(u32::MAX);
                total_risk[r][c] = cmp::min(total_risk[r][c], new);
            }
        }

        if total_risk == old {
            break;
        }
    }
    return *total_risk.last().unwrap().last().unwrap();
}

pub fn part_2(input: &str) -> u32 {
    let top_left_costs: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let stuff: Vec<Vec<Vec<Vec<u32>>>> = (0..5)
        .map(|r| {
            (0..5)
                .map(|c| {
                    top_left_costs
                        .iter()
                        .map(|row| row.iter().map(|val| ((val + r + c - 1) % 9) + 1).collect())
                        .collect()
                })
                .collect()
        })
        .collect();

    let costs: Vec<Vec<u32>> = stuff
        .iter()
        .flat_map(|big_rows| {
            (0..big_rows[0].len()).map(|i| {
                big_rows
                    .iter()
                    .flat_map(|thing| thing[i].iter().map(|&x| x))
                    .collect::<Vec<u32>>()
            })
        })
        .collect();

    let mut total_risk = vec![vec![u32::MAX; costs[0].len()]; costs.len()];
    total_risk[0][0] = 0;

    loop {
        let old: Vec<Vec<u32>> = total_risk
            .iter()
            .map(|r| r.iter().map(|&i| i).collect())
            .collect();

        for r in 0..total_risk.len() {
            for c in 0..(total_risk[r].len()) {
                let above: Option<u32> = r
                    .checked_sub(1)
                    .and_then(|up| total_risk.get(up).map(|row| row[c]));
                let below: Option<u32> = total_risk.get(r + 1).map(|row| row[c]);
                let left: Option<u32> = c
                    .checked_sub(1)
                    .and_then(|across| total_risk.get(r).and_then(|row| row.get(across)))
                    .map(|&x| x);
                let right: Option<u32> =
                    total_risk.get(r).and_then(|row| row.get(c + 1)).map(|&x| x);
                let new = [above, below, left, right]
                    .iter()
                    .flat_map(|x| x.iter())
                    .min()
                    .unwrap()
                    .checked_add(costs[r][c])
                    .unwrap_or(u32::MAX);
                total_risk[r][c] = cmp::min(total_risk[r][c], new);
            }
        }

        if total_risk == old {
            break;
        }
    }
    return *total_risk.last().unwrap().last().unwrap();
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(40, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(315, part_2(input::TEST_INPUT));
    }
}
