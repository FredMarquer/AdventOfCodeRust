use std::mem;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day16 {
    let input = include_str!("inputs/16.txt");
    let data = hex::decode(input).unwrap();
    let mut binary_reader = BinaryReader::from_buffer(&data);
    let packet = Packet::from_binary_reader(&mut binary_reader);

    Day16 { packet }
}

pub struct Day16 {
    packet: Packet
}

impl Solver for Day16 {
    fn run_part1(&self) -> SolverResult {
        self.packet.sum_verions().into()
    }

    fn run_part2(&self) -> SolverResult {
        self.packet.evaluate().into()
    }
}

struct Packet {
    version: usize,
    packet_type: PacketType,
    literal_value: u64,
    sub_packets: Box<[Packet]>,
}

impl Packet {
    fn from_binary_reader(binary_reader: &mut BinaryReader) -> Packet {
        let version = binary_reader.read_bits(3);
        let packet_type = binary_reader.read_bits(3);
        let packet_type = PacketType::from_usize(packet_type);
        let mut literal_value = 0;
        let mut sub_packets = Vec::new();
        match packet_type {
            PacketType::LiteralValue => {
                loop {
                    let last = binary_reader.read_bits(1) == 0;
                    literal_value |= binary_reader.read_bits(4) as u64;
                    if last {
                        break;
                    }
                    literal_value <<= 4;
                }
            }
            _=> {
                let length_type_id = binary_reader.read_bits(1);
                if length_type_id == 0 {
                    let sub_packets_length = binary_reader.read_bits(15);
                    let end_position = binary_reader.bit_position() + sub_packets_length;
                    loop {
                        let sub_packet = Packet::from_binary_reader(binary_reader);
                        sub_packets.push(sub_packet);
                        assert!(binary_reader.bit_position() <= end_position);
                        if binary_reader.bit_position() == end_position {
                            break;
                        }
                    }
                } else {
                    let sub_packets_count = binary_reader.read_bits(11);
                    for _ in 0..sub_packets_count {
                        let sub_packet = Packet::from_binary_reader(binary_reader);
                        sub_packets.push(sub_packet);
                    }
                }
            }
        }

        Packet {
            version,
            packet_type,
            literal_value,
            sub_packets: sub_packets.into_boxed_slice()
        }
    }

    fn sum_verions(&self) -> usize {
        self.sub_packets.iter().fold(self.version, |sum, sub_packet| sum + sub_packet.sum_verions())
    }

    fn evaluate(&self) -> u64 {
        match self.packet_type {
            PacketType::Sum => self.sub_packets.iter().fold(0, |sum, sub_packet| sum + sub_packet.evaluate()),
            PacketType::Product => self.sub_packets.iter().fold(1, | value, sub_packet| value * sub_packet.evaluate()),
            PacketType::Minimum => self.sub_packets.iter().fold(u64::MAX, |min, sub_packet| u64::min(min, sub_packet.evaluate())),
            PacketType::Maximum => self.sub_packets.iter().fold(u64::MIN, |max, sub_packet| u64::max(max, sub_packet.evaluate())),
            PacketType::LiteralValue => self.literal_value,
            PacketType::GreaterThan => (self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate()) as u64,
            PacketType::LessThan => (self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate()) as u64,
            PacketType::EqualTo => (self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate()) as u64,
        }
    }
}

#[derive(PartialEq)]
enum PacketType {
    Sum             = 0,
    Product         = 1,
    Minimum         = 2,
    Maximum         = 3,
    LiteralValue    = 4,
    GreaterThan     = 5,
    LessThan        = 6,
    EqualTo         = 7,
}

impl PacketType {
    fn from_usize(value: usize) -> PacketType {
        match value {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            4 => PacketType::LiteralValue,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::EqualTo,
            _ => panic!("invalid value: {}", value),
        }
    }
}

struct BinaryReader<'a> {
    buffer: &'a [u8],
    position: usize,
    bit_index: usize,
}

impl<'a> BinaryReader<'a> {
    fn from_buffer(buffer: &'a [u8]) -> BinaryReader {
        BinaryReader {
            buffer,
            bit_index: 0,
            position: 0,
        }
    }

    fn bit_position(&self) -> usize {
        self.position * 8 + self.bit_index
    }

    fn read_bits(&mut self, bits_count: usize) -> usize {
        assert!(bits_count > 0 && bits_count <= mem::size_of::<usize>() * 8);

        let mut reaming_bits_to_read = bits_count;
        let mut value = 0;
        while reaming_bits_to_read != 0 {
            let bits_to_read_for_position = usize::min(reaming_bits_to_read, 8 - self.bit_index);
            reaming_bits_to_read -= bits_to_read_for_position;

            let offset = 8 - (self.bit_index + bits_to_read_for_position);
            let mask = (1 << bits_to_read_for_position) - 1;
            let bits = (self.buffer[self.position] as usize >> offset) & mask;
            value |= bits << reaming_bits_to_read;

            self.bit_index += bits_to_read_for_position;
            if self.bit_index == 8 {
                self.bit_index = 0;
                self.position += 1;
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 993.into(), "Part1");
        assert_eq!(day.run_part2(), 144595909277_i64.into(), "Part2");
    }
}