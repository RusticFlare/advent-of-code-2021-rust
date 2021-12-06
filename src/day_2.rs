use std::str::FromStr;
pub mod input;

#[derive(Default)]
struct Position {
    horizontal: u32,
    depth: u32,
}

impl Position {
    fn result(self) -> u32 {
        self.horizontal * self.depth
    }
}

#[derive(Default)]
struct PositionWithAim {
    position: Position,
    aim: u32,
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Command {
    fn execute_1(self, p: Position) -> Position {
        match self {
            Command::Forward(v) => Position {
                horizontal: p.horizontal + v,
                ..p
            },
            Command::Up(v) => Position {
                depth: p.depth - v,
                ..p
            },
            Command::Down(v) => Position {
                depth: p.depth + v,
                ..p
            },
        }
    }

    fn execute_2(self, p: PositionWithAim) -> PositionWithAim {
        match self {
            Command::Forward(v) => PositionWithAim {
                position: Position {
                    horizontal: p.position.horizontal + v,
                    depth: p.position.depth + (p.aim * v),
                },
                ..p
            },
            Command::Up(v) => PositionWithAim {
                aim: p.aim - v,
                ..p
            },
            Command::Down(v) => PositionWithAim {
                aim: p.aim + v,
                ..p
            },
        }
    }
}

#[derive(Debug)]
struct ParseCommandError {}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_parts: Vec<&str> = s.split_whitespace().collect();
        let value = command_parts[1].parse::<u32>().unwrap();
        match command_parts[0] {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err(ParseCommandError {}),
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .fold(Default::default(), |acc, command| command.execute_1(acc))
        .result()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .fold(Default::default(), |acc, command| command.execute_2(acc))
        .position
        .result()
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(150, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(900, part_2(input::TEST_INPUT));
    }
}
