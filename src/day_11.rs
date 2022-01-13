pub mod input;

pub fn part_1(input: &str) -> usize {
    let mut octopuses = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut flashes = 0;

    for _ in 0..100 {
        octopuses = next(octopuses);
        flashes += octopuses
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&o| o == 0)
            .count();
    }
    return flashes;
}

fn next(octopuses: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut next = octopuses
        .iter()
        .map(|row| row.iter().map(|o| o + 1).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    loop {
        let tmp = next
            .iter()
            .map(|row| row.iter().map(|&i| i).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        next = flashes(next);
        if next == tmp {
            break;
        }
    }
    return next;
}

fn flashes(octopuses: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut next = octopuses
        .iter()
        .map(|row| row.iter().map(|&i| i).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    for r in 0..next.len() {
        for c in 0..next[r].len() {
            if next[r][c] > 9 {
                next[r][c] = 0;
                let row_before = r.checked_sub(1);
                let col_before = c.checked_sub(1);
                let row_after = r.checked_add(1).filter(|&i| i < next.len());
                let col_after = c.checked_add(1).filter(|&i| i < next[0].len());

                if row_before.is_some() {
                    let y = row_before.unwrap();
                    if col_before.is_some() {
                        let x = col_before.unwrap();
                        if next[y][x] > 0 {
                            next[y][x] += 1;
                        };
                    };
                    let x = c;
                    if next[y][x] > 0 {
                        next[y][x] += 1;
                    };
                    if col_after.is_some() {
                        let x = col_after.unwrap();
                        if next[y][x] > 0 {
                            next[y][x] += 1;
                        };
                    };
                };
                if col_before.is_some() {
                    let y = r;
                    let x = col_before.unwrap();
                    if next[y][x] > 0 {
                        next[y][x] += 1;
                    };
                };
                if col_after.is_some() {
                    let y = r;
                    let x = col_after.unwrap();
                    if next[y][x] > 0 {
                        next[y][x] += 1;
                    };
                };
                if row_after.is_some() {
                    let y = row_after.unwrap();
                    if col_before.is_some() {
                        let x = col_before.unwrap();
                        if next[y][x] > 0 {
                            next[y][x] += 1;
                        };
                    };
                    let x = c;
                    if next[y][x] > 0 {
                        next[y][x] += 1;
                    };
                    if col_after.is_some() {
                        let x = col_after.unwrap();
                        if next[y][x] > 0 {
                            next[y][x] += 1;
                        };
                    };
                };
            };
        }
    }
    return next;
}

pub fn part_2(input: &str) -> usize {
    let mut octopuses = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for i in 0..9999999999 {
        octopuses = next(octopuses);
        if octopuses
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&o| o == 0)
            .count() == 100 {
            return i + 1;
        };
    };
    return 0;
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(1656, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(195, part_2(input::TEST_INPUT));
    }
}
