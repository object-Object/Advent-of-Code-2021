use std::{
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
struct ParseErr(String);

impl From<ParseIntError> for ParseErr {
    fn from(e: ParseIntError) -> Self {
        ParseErr(e.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketType {
    Literal,
    SumOp,
    ProductOp,
    MinOp,
    MaxOp,
    GTOp,
    LTOp,
    EqualOp,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    version: u64,
    packet_type: PacketType,
    bitstring_length: usize,
    value: Option<u64>,
    sub_packets: Option<Vec<Packet>>,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        self.version
            + match &self.sub_packets {
                Some(sub_packets) => sub_packets.iter().map(|p| p.version_sum()).sum(),
                None => 0,
            }
    }

    fn evaluate(&self) -> u64 {
        let default = Vec::new();
        let mut sub_values = self
            .sub_packets
            .as_ref()
            .unwrap_or(&default)
            .iter()
            .map(|p| p.evaluate());

        match self.packet_type {
            PacketType::Literal => self.value.unwrap(),
            PacketType::SumOp => sub_values.sum(),
            PacketType::ProductOp => sub_values.product(),
            PacketType::MinOp => sub_values.min().unwrap(),
            PacketType::MaxOp => sub_values.max().unwrap(),
            PacketType::GTOp => (sub_values.next() > sub_values.next()) as u64,
            PacketType::LTOp => (sub_values.next() < sub_values.next()) as u64,
            PacketType::EqualOp => (sub_values.next() == sub_values.next()) as u64,
        }
    }
}

impl FromStr for Packet {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version = u64::from_str_radix(&s[0..3], 2)?;
        Ok(match u32::from_str_radix(&s[3..6], 2)? {
            4 => {
                let mut group_index = 6;
                let mut value_bitstring = "".to_string();
                loop {
                    value_bitstring = format!(
                        "{}{}",
                        value_bitstring,
                        s[(group_index + 1)..(group_index + 5)].to_string()
                    );
                    if s.chars().nth(group_index) == Some('0') {
                        break;
                    }
                    group_index += 5;
                }
                let value = Some(u64::from_str_radix(&value_bitstring, 2)?);
                Packet {
                    version,
                    packet_type: PacketType::Literal,
                    bitstring_length: group_index + 5,
                    value,
                    sub_packets: None,
                }
            }
            type_id => {
                let mut sub_packets = Vec::new();
                let mut bitstring_length = 0;
                if s.chars()
                    .nth(6)
                    .ok_or_else(|| ParseErr("no 6th bit".to_string()))?
                    == '0'
                {
                    bitstring_length += 22;
                    let mut remaining_bits = usize::from_str_radix(&s[7..22], 2)?;
                    while remaining_bits > 0 {
                        let next = s[bitstring_length..].parse::<Packet>()?;
                        bitstring_length += next.bitstring_length;
                        remaining_bits -= next.bitstring_length;
                        sub_packets.push(next);
                    }
                } else {
                    bitstring_length += 18;
                    let num_sub_packets = u32::from_str_radix(&s[7..18], 2)?;
                    for _ in 0..num_sub_packets {
                        let next = s[bitstring_length..].parse::<Packet>()?;
                        bitstring_length += next.bitstring_length;
                        sub_packets.push(next);
                    }
                }
                let sub_packets = Some(sub_packets);
                Packet {
                    version,
                    packet_type: match type_id {
                        0 => PacketType::SumOp,
                        1 => PacketType::ProductOp,
                        2 => PacketType::MinOp,
                        3 => PacketType::MaxOp,
                        5 => PacketType::GTOp,
                        6 => PacketType::LTOp,
                        7 => PacketType::EqualOp,
                        _ => unreachable!(),
                    },
                    bitstring_length,
                    value: None,
                    sub_packets,
                }
            }
        })
    }
}

impl TryFrom<&str> for Packet {
    type Error = ParseErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .map(|c| {
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
                    _ => unreachable!(),
                }
                .to_string()
            })
            .reduce(|acc, s| format!("{}{}", acc, s))
            .ok_or_else(|| ParseErr("string was empty".to_string()))?
            .parse()
    }
}

fn read_file(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().next().unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let packet: Packet = "D2FE28".try_into().unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 6,
                packet_type: PacketType::Literal,
                bitstring_length: 21,
                value: Some(2021),
                sub_packets: None,
            }
        )
    }

    #[test]
    fn operator_length_type_0() {
        let packet: Packet = "38006F45291200".try_into().unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.packet_type, PacketType::LTOp);
        assert_eq!(packet.sub_packets.as_ref().unwrap()[0].value, Some(10));
        assert_eq!(packet.sub_packets.as_ref().unwrap()[1].value, Some(20));
    }

    #[test]
    fn operator_length_type_1() {
        let packet: Packet = "EE00D40C823060".try_into().unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(packet.packet_type, PacketType::MaxOp);
        assert_eq!(packet.sub_packets.as_ref().unwrap()[0].value, Some(1));
        assert_eq!(packet.sub_packets.as_ref().unwrap()[1].value, Some(2));
        assert_eq!(packet.sub_packets.as_ref().unwrap()[2].value, Some(3));
    }

    #[test]
    fn version_sum_16() {
        let packet: Packet = "8A004A801A8002F478".try_into().unwrap();
        assert_eq!(packet.version_sum(), 16);
    }

    #[test]
    fn version_sum_12() {
        let packet: Packet = "620080001611562C8802118E34".try_into().unwrap();
        assert_eq!(packet.version_sum(), 12);
    }

    #[test]
    fn version_sum_23() {
        let packet: Packet = "C0015000016115A2E0802F182340".try_into().unwrap();
        assert_eq!(packet.version_sum(), 23);
    }

    #[test]
    fn version_sum_31() {
        let packet: Packet = "A0016C880162017C3686B18A3D4780".try_into().unwrap();
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn sum_op() {
        let packet: Packet = "C200B40A82".try_into().unwrap();
        assert_eq!(packet.evaluate(), 3);
    }

    #[test]
    fn product_op() {
        let packet: Packet = "04005AC33890".try_into().unwrap();
        assert_eq!(packet.evaluate(), 54);
    }

    #[test]
    fn min_op() {
        let packet: Packet = "880086C3E88112".try_into().unwrap();
        assert_eq!(packet.evaluate(), 7);
    }

    #[test]
    fn max_op() {
        let packet: Packet = "CE00C43D881120".try_into().unwrap();
        assert_eq!(packet.evaluate(), 9);
    }

    #[test]
    fn lt_op() {
        let packet: Packet = "D8005AC2A8F0".try_into().unwrap();
        assert_eq!(packet.evaluate(), 1);
    }

    #[test]
    fn gt_op() {
        let packet: Packet = "F600BC2D8F".try_into().unwrap();
        assert_eq!(packet.evaluate(), 0);
    }

    #[test]
    fn equal_op() {
        let packet: Packet = "9C005AC2F8F0".try_into().unwrap();
        assert_eq!(packet.evaluate(), 0);
    }

    #[test]
    fn multiple_ops() {
        let packet: Packet = "9C0141080250320F1802104A08".try_into().unwrap();
        assert_eq!(packet.evaluate(), 1);
    }
}

fn main() {
    let packet: Packet = read_file("data/input.txt").as_str().try_into().unwrap();

    // part 1
    println!("Part 1: version_sum = {}", packet.version_sum());

    // part 2
    println!("Part 2: evaluate = {}", packet.evaluate());
}
