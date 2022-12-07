use anyhow::*;
use aoc_common::hex_to_binary_string;
use aoc_common::run;

fn main() -> Result<()> {
    run(parse, part1, part2)
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Literal,
    Operation(Operator),
}

impl From<usize> for PacketType {
    fn from(ptype: usize) -> PacketType {
        match ptype {
            4 => PacketType::Literal,
            op => PacketType::Operation(Operator::from(op)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<usize> for Operator {
    fn from(op: usize) -> Operator {
        match op {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualTo,
            _ => panic!("operator"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LengthType {
    SubPacketLength,
    SubPacketCount,
}

impl From<usize> for LengthType {
    fn from(ltype: usize) -> LengthType {
        match ltype {
            0 => LengthType::SubPacketLength,
            1 => LengthType::SubPacketCount,
            _ => panic!("length type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operation {
        version: usize,
        op: Operator,
        inner: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operation { version, inner, .. } => {
                *version + inner.iter().map(|x| x.version_sum()).sum::<usize>()
            }
        }
    }

    fn execute(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operation { op, inner, .. } => match op {
                Operator::Sum => inner.iter().map(|x| x.execute()).sum(),
                Operator::Product => inner.iter().map(|x| x.execute()).product(),
                Operator::Minimum => inner.iter().map(|x| x.execute()).min().unwrap(),
                Operator::Maximum => inner.iter().map(|x| x.execute()).max().unwrap(),
                Operator::GreaterThan => {
                    let operands: Vec<usize> = inner.iter().map(|x| x.execute()).collect();
                    usize::from(operands[0] > operands[1])
                }
                Operator::LessThan => {
                    let operands: Vec<usize> = inner.iter().map(|x| x.execute()).collect();
                    usize::from(operands[0] < operands[1])
                }
                Operator::EqualTo => {
                    let operands: Vec<usize> = inner.iter().map(|x| x.execute()).collect();
                    usize::from(operands[0] == operands[1])
                }
            },
        }
    }
}

struct PacketParser {
    bits: String,
    pos: usize,
}

impl PacketParser {
    fn new(bits: String) -> PacketParser {
        PacketParser { bits, pos: 0 }
    }

    fn read_bits(&mut self, len: usize) -> Result<usize> {
        let bits = &self.bits[self.pos..self.pos + len];
        self.pos += len;

        Ok(usize::from_str_radix(bits, 2)?)
    }

    fn parse_packet(&mut self) -> Result<Packet> {
        let version = self.read_bits(3)?;
        let ptype = PacketType::from(self.read_bits(3)?);

        match ptype {
            PacketType::Literal => {
                let mut value = 0;
                loop {
                    let last_segment = self.read_bits(1)?;
                    let bits = self.read_bits(4)?;
                    value <<= 4;
                    value |= bits;
                    if last_segment == 0 {
                        break;
                    }
                }
                Ok(Packet::Literal { version, value })
            }
            PacketType::Operation(op) => {
                let length_type = LengthType::from(self.read_bits(1)?);
                let mut inner = vec![];
                match length_type {
                    LengthType::SubPacketLength => {
                        let count = self.read_bits(15)?;
                        let start_pos = self.pos;
                        while self.pos < start_pos + count {
                            let inner_packet = self.parse_packet()?;
                            inner.push(inner_packet);
                        }
                    }
                    LengthType::SubPacketCount => {
                        let count = self.read_bits(11)?;
                        while inner.len() < count {
                            let inner_packet = self.parse_packet()?;
                            inner.push(inner_packet);
                        }
                    }
                }
                Ok(Packet::Operation { version, op, inner })
            }
        }
    }
}

fn parse(contents: &str) -> Result<Packet> {
    PacketParser::new(hex_to_binary_string(contents.trim())).parse_packet()
}

fn part1(contents: &Packet) -> Result<usize> {
    Ok(contents.version_sum())
}

fn part2(contents: &Packet) -> Result<usize> {
    Ok(contents.execute())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal_packet() -> Result<()> {
        match parse(LITERAL_PACKET)? {
            Packet::Literal { version, value } => {
                assert_eq!(version, 6);
                assert_eq!(value, 2021);
            }
            _ => bail!("incorrect packet"),
        }

        Ok(())
    }

    #[test]
    fn parse_operator_packet_length_type() -> Result<()> {
        match parse(OPERATOR_LENGTH_PACKET)? {
            Packet::Operation { version, inner, .. } => {
                assert_eq!(version, 1);
                assert_eq!(inner.len(), 2);
                assert_eq!(
                    inner[0],
                    Packet::Literal {
                        version: 6,
                        value: 10
                    }
                );
                assert_eq!(
                    inner[1],
                    Packet::Literal {
                        version: 2,
                        value: 20
                    }
                );
            }
            _ => bail!("incorrect packet"),
        }

        Ok(())
    }

    #[test]
    fn parse_operator_packet_count_type() -> Result<()> {
        match parse(OPERATOR_COUNT_PACKET)? {
            Packet::Operation { version, inner, .. } => {
                assert_eq!(version, 7);
                assert_eq!(inner.len(), 3);
                assert_eq!(
                    inner[0],
                    Packet::Literal {
                        version: 2,
                        value: 1
                    }
                );
                assert_eq!(
                    inner[1],
                    Packet::Literal {
                        version: 4,
                        value: 2
                    }
                );
                assert_eq!(
                    inner[2],
                    Packet::Literal {
                        version: 1,
                        value: 3
                    }
                );
            }
            _ => bail!("incorrect packet"),
        }

        Ok(())
    }

    #[test]
    fn test_sample1_part1() -> Result<()> {
        assert_eq!(part1(&parse(SAMPLE_PACKET_1)?)?, 16);

        Ok(())
    }

    #[test]
    fn test_sample2_part1() -> Result<()> {
        assert_eq!(part1(&parse(SAMPLE_PACKET_2)?)?, 12);

        Ok(())
    }

    #[test]
    fn test_sample3_part1() -> Result<()> {
        assert_eq!(part1(&parse(SAMPLE_PACKET_3)?)?, 23);

        Ok(())
    }

    #[test]
    fn test_sample4_part1() -> Result<()> {
        let packet = parse(SAMPLE_PACKET_4)?;
        assert_eq!(part1(&packet)?, 31);

        Ok(())
    }

    #[test]
    fn test_sum_packet() -> Result<()> {
        assert_eq!(part2(&parse(SUM_PACKET)?)?, 3);

        Ok(())
    }

    #[test]
    fn test_mul_packet() -> Result<()> {
        assert_eq!(part2(&parse(MUL_PACKET)?)?, 54);

        Ok(())
    }

    #[test]
    fn test_min_packet() -> Result<()> {
        assert_eq!(part2(&parse(MIN_PACKET)?)?, 7);

        Ok(())
    }

    #[test]
    fn test_max_packet() -> Result<()> {
        assert_eq!(part2(&parse(MAX_PACKET)?)?, 9);

        Ok(())
    }

    #[test]
    fn test_lt_packet() -> Result<()> {
        assert_eq!(part2(&parse(LT_PACKET)?)?, 1);

        Ok(())
    }

    #[test]
    fn test_gt_packet() -> Result<()> {
        assert_eq!(part2(&parse(GT_PACKET)?)?, 0);

        Ok(())
    }

    #[test]
    fn test_ne_packet() -> Result<()> {
        assert_eq!(part2(&parse(NE_PACKET)?)?, 0);

        Ok(())
    }

    #[test]
    fn test_eq_packet() -> Result<()> {
        assert_eq!(part2(&parse(EQ_PACKET)?)?, 1);

        Ok(())
    }

    const LITERAL_PACKET: &str = "D2FE28";
    const OPERATOR_LENGTH_PACKET: &str = "38006F45291200";
    const OPERATOR_COUNT_PACKET: &str = "EE00D40C823060";

    const SAMPLE_PACKET_1: &str = "8A004A801A8002F478";
    const SAMPLE_PACKET_2: &str = "620080001611562C8802118E34";
    const SAMPLE_PACKET_3: &str = "C0015000016115A2E0802F182340";
    const SAMPLE_PACKET_4: &str = "A0016C880162017C3686B18A3D4780";

    const SUM_PACKET: &str = "C200B40A82"; // finds the sum of 1 and 2, resulting in the value 3.
    const MUL_PACKET: &str = "04005AC33890"; // finds the product of 6 and 9, resulting in the value 54.
    const MIN_PACKET: &str = "880086C3E88112"; // finds the minimum of 7, 8, and 9, resulting in the value 7.
    const MAX_PACKET: &str = "CE00C43D881120"; // finds the maximum of 7, 8, and 9, resulting in the value 9.
    const LT_PACKET: &str = "D8005AC2A8F0"; // produces 1, because 5 is less than 15.
    const GT_PACKET: &str = "F600BC2D8F"; // produces 0, because 5 is not greater than 15.
    const NE_PACKET: &str = "9C005AC2F8F0"; // produces 0, because 5 is not equal to 15.
    const EQ_PACKET: &str = "9C0141080250320F1802104A08"; // produces 1, because 1 + 3 = 2 * 2.
}
