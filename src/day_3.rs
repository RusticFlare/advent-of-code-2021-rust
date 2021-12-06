pub mod input;

pub fn part_1(input: &str) -> u32 {
    let (gamma_rate_bits, epsilon_rate_bits) = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| if c == '1' { 1 } else { -1 })
                .collect::<Vec<i32>>()
        })
        .reduce(|acc, values| {
            acc.iter()
                .zip(values.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<i32>>()
        })
        .unwrap()
        .iter()
        .map(|&v| if v > 0 { ('1', '0') } else { ('0', '1') })
        .unzip::<char, char, Vec<char>, Vec<char>>();
    let gamma_rate = u32::from_str_radix(&gamma_rate_bits.iter().collect::<String>(), 2).unwrap();
    let epsilon_rate =
        u32::from_str_radix(&epsilon_rate_bits.iter().collect::<String>(), 2).unwrap();
    return gamma_rate * epsilon_rate;
}

struct OxygenGeneratorRating {
    head: String,
    tails: Vec<String>,
}

struct Co2ScrubberRating {
    head: String,
    tails: Vec<String>,
}

impl OxygenGeneratorRating {
    fn value(self) -> u32 {
        u32::from_str_radix(&(self.head + &self.tails[0][..]), 2).unwrap()
    }

    fn next(self) -> OxygenGeneratorRating {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = self
            .tails
            .iter()
            .map(|s| &s[..])
            .partition(|s| s.starts_with("1"));
        if ones.len() >= zeros.len() {
            OxygenGeneratorRating {
                head: self.head + "1",
                tails: ones.iter().map(|s| s[1..s.len()].to_string()).collect(),
            }
        } else {
            OxygenGeneratorRating {
                head: self.head + "0",
                tails: zeros.iter().map(|s| s[1..s.len()].to_string()).collect(),
            }
        }
    }
}

impl Co2ScrubberRating {
    fn value(self) -> u32 {
        u32::from_str_radix(&(self.head + &self.tails[0]), 2).unwrap()
    }

    fn next(self) -> Co2ScrubberRating {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = self
            .tails
            .iter()
            .map(|s| &s[..])
            .partition(|s| s.starts_with("1"));
        if ones.len() < zeros.len() {
            Co2ScrubberRating {
                head: self.head + "1",
                tails: ones.iter().map(|s| s[1..s.len()].to_string()).collect(),
            }
        } else {
            Co2ScrubberRating {
                head: self.head + "0",
                tails: zeros.iter().map(|s| s[1..s.len()].to_string()).collect(),
            }
        }
    }
}

pub fn part_2(input: &str) -> u32 {
    let mut x = OxygenGeneratorRating {
        head: "".to_string(),
        tails: input.lines().map(|s| s.to_string()).collect(),
    };

    while x.tails.len() > 1 {
        x = x.next();
    }

    let mut y = Co2ScrubberRating {
        head: "".to_string(),
        tails: input.lines().map(|s| s.to_string()).collect(),
    };

    while y.tails.len() > 1 {
        y = y.next();
    }

    return x.value() * y.value();
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(198, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(230, part_2(input::TEST_INPUT));
    }
}
