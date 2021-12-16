//! Day 16: Packet Decoder
use aoc21::{util::*, Quizzer};

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        parser::parse(&base16_decode(input.trim()).unwrap())
            .unwrap()
            .version_sum()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        parser::parse(&base16_decode(input.trim()).unwrap())
            .unwrap()
            .evaluate()
            .to_string()
    }
}

/// The supported operators
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
    Invalid,
}

impl From<u8> for Operator {
    fn from(value: u8) -> Self {
        match value {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Min,
            3 => Operator::Max,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::Equal,
            _ => Operator::Invalid,
        }
    }
}

/// The different kinds of data a packet can contain
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TypeData {
    /// A literal packet
    Literal(u64),
    /// An operator packet
    Operator {
        /// The specific operator of the packet
        op: Operator,
        /// The subpackets
        packets: Vec<Packet>,
    },
}

/// A BITS-Packet as transmitted by the elves
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Packet {
    /// Packet version
    version: u8,
    /// Packet payload
    data: TypeData,
}

impl Packet {
    /// Calculate the sum of the packets and subpackets versions
    pub(crate) fn version_sum(&self) -> u64 {
        self.version as u64
            + match &self.data {
                TypeData::Literal(_) => 0,
                TypeData::Operator { packets, .. } => {
                    packets.iter().map(|p| p.version_sum()).sum::<u64>()
                }
            }
    }

    /// Evaluate the packet
    pub(crate) fn evaluate(&self) -> u64 {
        use self::Operator::*;
        use TypeData::*;

        match &self.data {
            Literal(l) => *l,
            Operator { op: Sum, packets } => packets.iter().map(Packet::evaluate).sum(),
            Operator {
                op: Product,
                packets,
            } => packets.iter().map(Packet::evaluate).product(),
            Operator { op: Min, packets } => packets
                .iter()
                .map(Packet::evaluate)
                .min()
                .unwrap_or_default(),
            Operator { op: Max, packets } => packets
                .iter()
                .map(Packet::evaluate)
                .max()
                .unwrap_or_default(),
            Operator {
                op: GreaterThan,
                packets,
            } => {
                if packets[0].evaluate() > packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Operator {
                op: LessThan,
                packets,
            } => {
                if packets[0].evaluate() < packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Operator { op: Equal, packets } => {
                if packets[0].evaluate() == packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Operator { op: Invalid, .. } => unreachable!(),
        }
    }
}

pub(crate) mod parser {
    //! A parser for BITS
    use super::{Operator, Packet, TypeData};
    use nom::bits::complete::{tag, take};
    use nom::branch::alt;
    use nom::combinator::{map, map_opt, verify};
    use nom::multi::{length_count, many_till};
    use nom::sequence::{preceded, tuple};

    /// Type alias for noms bit slices
    type Bits<'input> = (&'input [u8], usize);
    /// Common result for all subparsers
    type IResult<'input, T> = nom::IResult<Bits<'input>, T>;
    /// Result of the modules top-level parse function
    type Result<'input> =
        std::result::Result<Packet, nom::Err<nom::error::Error<(&'input [u8], usize)>>>;

    /// A packets header
    struct Header {
        /// Packet version
        version: u8,
        /// Packet type
        type_id: u8,
    }

    /// Parse the version bits
    fn version(input: Bits) -> IResult<u8> {
        take(3usize)(input)
    }

    /// Parse the type id
    fn type_id(input: Bits) -> IResult<u8> {
        take(3usize)(input)
    }

    /// Parse the packet header
    fn header(input: Bits) -> IResult<Header> {
        map(tuple((version, type_id)), |(version, type_id)| Header {
            version,
            type_id,
        })(input)
    }

    /// Parse a literal packet
    fn literal(input: Bits) -> IResult<Packet> {
        // A datum is always prefixed by either a 1 or a 0 bit
        let one = |input| tag(1, 1usize)(input);
        let zero = |input| tag(0, 1usize)(input);
        let datum = |input| take(4usize)(input);

        let data = move |input| {
            map(
                many_till(preceded(one, datum), preceded(zero, datum)),
                |(mut data, last_data)| {
                    data.push(last_data);
                    data.iter()
                        .fold(0, |value, datum| (value << 4) | (datum & 0x0F) as u64)
                },
            )(input)
        };

        map(
            tuple((verify(header, |header| header.type_id == 4), data)),
            |(header, value)| Packet {
                version: header.version,
                data: TypeData::Literal(value),
            },
        )(input)
    }

    /// Parse a operator packet
    fn operator(input: Bits) -> IResult<Packet> {
        let packets = |input| {
            let (input, total): (_, u16) = take(15usize)(input)?;

            let mut length = 0u16;
            let mut packets = vec![];
            let mut input = input;

            while total > length {
                let (remaining, packet) = packet(input)?;
                length += delta_bits(input, remaining) as u16;
                packets.push(packet);
                input = remaining;
            }

            Ok((input, packets))
        };

        let subpackets_from_bits = preceded(tag(0, 1usize), packets);
        let subpackets_from_count = preceded(
            tag(1, 1usize),
            length_count(take::<_, u16, _, _>(11usize), packet),
        );

        let subpackets = alt((subpackets_from_bits, subpackets_from_count));

        map_opt(
            tuple((
                verify(header, |header| {
                    Operator::from(header.type_id) != Operator::Invalid
                }),
                subpackets,
            )),
            |(header, packets)| {
                Some(Packet {
                    version: header.version,
                    data: TypeData::Operator {
                        op: header.type_id.into(),
                        packets,
                    },
                })
            },
        )(input)
    }

    /// Calculate how many bits have been parsed between before and after
    fn delta_bits(before: Bits, after: Bits) -> usize {
        (before.0.len() * 8 - before.1) - (after.0.len() * 8 - after.1)
    }

    /// Parse a packet
    fn packet(input: Bits) -> IResult<Packet> {
        alt((literal, operator))(input)
    }

    /// Top-level parsing function
    pub(crate) fn parse(input: &[u8]) -> Result<'_> {
        packet((input, 0)).map(|(_, p)| p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES1: &[&str] = &[
        "D2FE28",
        "38006F45291200",
        "EE00D40C823060",
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];

    const EXAMPLES2: &[(&str, u64)] = &[
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];

    #[test]
    fn part1_examples() {
        let parsed: Result<Vec<_>, _> = EXAMPLES1.iter().map(|x| base16_decode(x)).collect();
        let parsed = parsed.unwrap();

        assert_eq!(parser::parse(&parsed[0]).unwrap().version_sum(), 6);
        assert_eq!(parser::parse(&parsed[1]).unwrap().version_sum(), 9);
        assert_eq!(parser::parse(&parsed[2]).unwrap().version_sum(), 14);
        assert_eq!(parser::parse(&parsed[3]).unwrap().version_sum(), 16);
        assert_eq!(parser::parse(&parsed[4]).unwrap().version_sum(), 12);
        assert_eq!(parser::parse(&parsed[5]).unwrap().version_sum(), 23);
        assert_eq!(parser::parse(&parsed[6]).unwrap().version_sum(), 31);

        assert_eq!(
            parser::parse(&parsed[0]),
            Ok(Packet {
                version: 6,
                data: TypeData::Literal(2021),
            })
        );

        assert_eq!(
            parser::parse(&parsed[1]),
            Ok(Packet {
                version: 1,
                data: TypeData::Operator {
                    op: Operator::LessThan,
                    packets: vec![
                        Packet {
                            version: 6,
                            data: TypeData::Literal(10),
                        },
                        Packet {
                            version: 2,
                            data: TypeData::Literal(20),
                        },
                    ],
                }
            })
        );

        assert_eq!(
            parser::parse(&parsed[2]),
            Ok(Packet {
                version: 7,
                data: TypeData::Operator {
                    op: Operator::Max,
                    packets: vec![
                        Packet {
                            version: 2,
                            data: TypeData::Literal(1),
                        },
                        Packet {
                            version: 4,
                            data: TypeData::Literal(2),
                        },
                        Packet {
                            version: 1,
                            data: TypeData::Literal(3),
                        },
                    ],
                }
            })
        );

        assert_eq!(
            parser::parse(&parsed[3]),
            Ok(Packet {
                version: 4,
                data: TypeData::Operator {
                    op: Operator::Min,
                    packets: vec![Packet {
                        version: 1,
                        data: TypeData::Operator {
                            op: Operator::Min,
                            packets: vec![Packet {
                                version: 5,
                                data: TypeData::Operator {
                                    op: Operator::Min,
                                    packets: vec![Packet {
                                        version: 6,
                                        data: TypeData::Literal(15),
                                    }]
                                }
                            }]
                        }
                    }]
                }
            })
        );

        assert_eq!(
            parser::parse(&parsed[4]),
            Ok(Packet {
                version: 3,
                data: TypeData::Operator {
                    op: Operator::Sum,
                    packets: vec![
                        Packet {
                            version: 0,
                            data: TypeData::Operator {
                                op: Operator::Sum,
                                packets: vec![
                                    Packet {
                                        version: 0,
                                        data: TypeData::Literal(10)
                                    },
                                    Packet {
                                        version: 5,
                                        data: TypeData::Literal(11)
                                    },
                                ]
                            }
                        },
                        Packet {
                            version: 1,
                            data: TypeData::Operator {
                                op: Operator::Sum,
                                packets: vec![
                                    Packet {
                                        version: 0,
                                        data: TypeData::Literal(12)
                                    },
                                    Packet {
                                        version: 3,
                                        data: TypeData::Literal(13)
                                    },
                                ]
                            }
                        }
                    ]
                }
            })
        );

        assert_eq!(
            parser::parse(&parsed[5]),
            Ok(Packet {
                version: 6,
                data: TypeData::Operator {
                    op: Operator::Sum,
                    packets: vec![
                        Packet {
                            version: 0,
                            data: TypeData::Operator {
                                op: Operator::Sum,
                                packets: vec![
                                    Packet {
                                        version: 0,
                                        data: TypeData::Literal(10)
                                    },
                                    Packet {
                                        version: 6,
                                        data: TypeData::Literal(11)
                                    },
                                ]
                            }
                        },
                        Packet {
                            version: 4,
                            data: TypeData::Operator {
                                op: Operator::Sum,
                                packets: vec![
                                    Packet {
                                        version: 7,
                                        data: TypeData::Literal(12)
                                    },
                                    Packet {
                                        version: 0,
                                        data: TypeData::Literal(13)
                                    },
                                ]
                            }
                        }
                    ]
                }
            })
        );

        assert_eq!(
            parser::parse(&parsed[6]),
            Ok(Packet {
                version: 5,
                data: TypeData::Operator {
                    op: Operator::Sum,
                    packets: vec![Packet {
                        version: 1,
                        data: TypeData::Operator {
                            op: Operator::Sum,
                            packets: vec![Packet {
                                version: 3,
                                data: TypeData::Operator {
                                    op: Operator::Sum,
                                    packets: vec![
                                        Packet {
                                            version: 7,
                                            data: TypeData::Literal(6),
                                        },
                                        Packet {
                                            version: 6,
                                            data: TypeData::Literal(6),
                                        },
                                        Packet {
                                            version: 5,
                                            data: TypeData::Literal(12),
                                        },
                                        Packet {
                                            version: 2,
                                            data: TypeData::Literal(15),
                                        },
                                        Packet {
                                            version: 2,
                                            data: TypeData::Literal(15),
                                        },
                                    ]
                                }
                            }]
                        }
                    }]
                }
            })
        );
    }

    #[test]
    fn part2_examples() {
        let parsed: Result<Vec<_>, _> = EXAMPLES2
            .iter()
            .map(|(packet, result)| (base16_decode(packet).map(|packet| (packet, *result))))
            .collect();
        let parsed = parsed.unwrap();

        assert_eq!(parser::parse(&parsed[0].0).unwrap().evaluate(), parsed[0].1);
        assert_eq!(parser::parse(&parsed[1].0).unwrap().evaluate(), parsed[1].1);
        assert_eq!(parser::parse(&parsed[2].0).unwrap().evaluate(), parsed[2].1);
        assert_eq!(parser::parse(&parsed[3].0).unwrap().evaluate(), parsed[3].1);
        assert_eq!(parser::parse(&parsed[4].0).unwrap().evaluate(), parsed[4].1);
        assert_eq!(parser::parse(&parsed[5].0).unwrap().evaluate(), parsed[5].1);
        assert_eq!(parser::parse(&parsed[6].0).unwrap().evaluate(), parsed[6].1);
    }
}
