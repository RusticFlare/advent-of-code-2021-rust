use crate::day_16::Packet::{Literal, Operator};
use std::u32;

pub mod input;

enum Packet {
    Literal {
        version: u32,
        packet_type_id: u8,
        value: u64,
    },
    Operator {
        version: u32,
        packet_type_id: u8,
        sub_packets: Vec<Packet>,
    },
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect::<String>()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn to_packet(input: &str) -> (Packet, usize) {
    if &input[3..6] == "100" {
        to_literal(input)
    } else {
        to_operator(input)
    }
}

fn to_literal(input: &str) -> (Packet, usize) {
    let mut index = 6;
    let mut binary_value: Vec<&str> = Vec::new();
    loop {
        binary_value.push(&input[(index + 1)..(index + 5)]);
        if input[index..(index + 1)].chars().next().unwrap() == '0' {
            index += 5;
            break;
        }
        index += 5;
    }
    (
        Literal {
            version: u32::from_str_radix(&input[0..3], 2).unwrap(),
            packet_type_id: 4,
            value: u64::from_str_radix(
                binary_value.iter().map(|&s| s).collect::<String>().as_str(),
                2,
            )
            .unwrap(),
        },
        index,
    )
}

fn to_operator(input: &str) -> (Packet, usize) {
    if input[6..7].chars().next().unwrap() == '0' {
        to_length_operator(input)
    } else {
        to_count_operator(input)
    }
}

fn to_length_operator(input: &str) -> (Packet, usize) {
    let length =
        usize::from_str_radix(input[7..22].chars().collect::<String>().as_str(), 2).unwrap();
    let mut index = 22;
    let mut sub_packets: Vec<Packet> = Vec::new();
    while index < (length + 22) {
        let (sub_packet, size) = to_packet(&input[index..]);
        sub_packets.push(sub_packet);
        index += size;
    }
    (
        Operator {
            version: u32::from_str_radix(&input[0..3], 2).unwrap(),
            packet_type_id: u8::from_str_radix(&input[4..6], 2).unwrap(),
            sub_packets,
        },
        index,
    )
}

fn to_count_operator(input: &str) -> (Packet, usize) {
    let count =
        usize::from_str_radix(input[7..18].chars().collect::<String>().as_str(), 2).unwrap();
    let mut index = 18;
    let mut sub_packets: Vec<Packet> = Vec::new();
    while sub_packets.len() < count {
        let (sub_packet, size) = to_packet(&input[index..]);
        sub_packets.push(sub_packet);
        index += size;
    }
    (
        Operator {
            version: u32::from_str_radix(&input[0..3], 2).unwrap(),
            packet_type_id: u8::from_str_radix(&input[4..6], 2).unwrap(),
            sub_packets,
        },
        index,
    )
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match &*self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => sub_packets
                .iter()
                .fold(*version, |p, q| p + q.version_sum()),
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let binary = convert_to_binary_from_hex(input);
    let (packet, _) = to_packet(binary.as_str());
    packet.version_sum()
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
        assert_eq!(31, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(input::TEST_INPUT));
    }
}
