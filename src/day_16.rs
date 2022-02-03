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
            packet_type_id: u8::from_str_radix(&input[3..6], 2).unwrap(),
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
            packet_type_id: u8::from_str_radix(&input[3..6], 2).unwrap(),
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

    fn evaluate(&self) -> u64 {
        match &*self {
            Packet::Literal {
                packet_type_id: 4,
                value,
                ..
            } => *value,
            Packet::Operator {
                packet_type_id: 0,
                sub_packets,
                ..
            } => sub_packets.iter().map(|p| p.evaluate()).sum(),
            Packet::Operator {
                packet_type_id: 1,
                sub_packets,
                ..
            } => sub_packets.iter().map(|p| p.evaluate()).product(),
            Packet::Operator {
                packet_type_id: 2,
                sub_packets,
                ..
            } => sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
            Packet::Operator {
                packet_type_id: 3,
                sub_packets,
                ..
            } => sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
            Packet::Operator {
                packet_type_id: 5,
                sub_packets,
                ..
            } => {
                if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Packet::Operator {
                packet_type_id: 6,
                sub_packets,
                ..
            } => {
                if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Packet::Operator {
                packet_type_id: 7,
                sub_packets,
                ..
            } => {
                if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Packet::Literal { packet_type_id, .. } => {
                println!("Literal CRY: {}", packet_type_id);
                0
            }
            Packet::Operator { packet_type_id, .. } => {
                println!("Operator CRY: {}", packet_type_id);
                0
            }
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let binary = convert_to_binary_from_hex(input);
    let (packet, _) = to_packet(binary.as_str());
    packet.version_sum()
}

pub fn part_2(input: &str) -> u64 {
    let binary = convert_to_binary_from_hex(input);
    let (packet, _) = to_packet(binary.as_str());
    packet.evaluate()
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(16, part_1("8A004A801A8002F478"));
        assert_eq!(12, part_1("620080001611562C8802118E34"));
        assert_eq!(23, part_1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part_1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3, part_2("C200B40A82"));
        assert_eq!(54, part_2("04005AC33890"));
        assert_eq!(7, part_2("880086C3E88112"));
        assert_eq!(9, part_2("CE00C43D881120"));
        assert_eq!(1, part_2("D8005AC2A8F0"));
        assert_eq!(0, part_2("F600BC2D8F"));
        assert_eq!(0, part_2("9C005AC2F8F0"));
        assert_eq!(1, part_2("9C0141080250320F1802104A08"));
    }
}
