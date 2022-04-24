use std::fs;

#[derive(Debug)]
enum TypeId {
    Sum,
    Prod,
    Min,
    Max,
    Lit,
    GreaterThan,
    LessThan,
    Equal,
}

impl TypeId {
    fn from_u8(value: u8) -> TypeId {
        match value {
            0 => TypeId::Sum,
            1 => TypeId::Prod,
            2 => TypeId::Min,
            3 => TypeId::Max,
            4 => TypeId::Lit,
            5 => TypeId::GreaterThan,
            6 => TypeId::LessThan,
            7 => TypeId::Equal,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: TypeId,

    data: PacketData,
}

fn char_to_bin(u: char) -> &'static str {
    match u {
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
         _  => "0000",
    }
}

fn hex_to_bin_str(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        result.push_str(char_to_bin(c));
    }

    result
}

fn parse_packet(input: String) -> (Packet, String) {
    let mut bits = input;

    let s: String = bits.drain(..3).collect();
    let version = u8::from_str_radix(s.as_str(), 2).unwrap();

    let s: String = bits.drain(..3).collect();
    let type_id = u8::from_str_radix(s.as_str(), 2).unwrap();

    // println!("version: {}", version);
    // println!("type_id: {}", type_id);

    let data =
        match type_id {
            4 => {
                let mut value_str = String::new();

                loop {
                    let s = bits.drain(..5).collect::<String>();

                    let is_end = s.starts_with("0");

                    value_str.push_str(
                        s.get(1..).unwrap()
                    );

                    if is_end {
                        break;
                    }
                }

                let value = u64::from_str_radix(value_str.as_str(), 2).unwrap();

                PacketData::Literal(value)
            },
            _ => {
                let mode = bits.drain(..1).collect::<String>();
                let mut subpackets: Vec<Packet> = vec![];

                if mode == "0" {
                    let size_str = bits.drain(..15).collect::<String>();
                    let size = usize::from_str_radix(size_str.as_str(), 2).unwrap();

                    let mut bits_packets = bits.drain(..size).collect::<String>();

                    while bits_packets.len() > 0 {
                        let (packet, remains) = parse_packet(bits_packets);

                        subpackets.push(packet);

                        bits_packets = remains;
                    }
                } else {
                    let size_str = bits.drain(..11).collect::<String>();
                    let mut size = usize::from_str_radix(size_str.as_str(), 2).unwrap();

                    while size > 0 {
                        let (packet, remains) = parse_packet(bits);

                        subpackets.push(packet);

                        bits = remains;

                        size -= 1;
                    }
                }

                PacketData::Operator(subpackets)
            }
        };

    // println!("*  version: {}", version);
    // println!("*  type_id: {}", type_id);

    let packet = Packet {
        version,
        type_id: TypeId::from_u8(type_id),
        data,
    };

    (packet, bits)
}

impl Packet {
    fn new(input: &str) -> Packet {
        let bits: String = hex_to_bin_str(input);

        let (packet, _) = parse_packet(bits);

        packet
    }

    fn sum_versions(&self) -> usize {
        let mut result = self.version as usize;

        if let PacketData::Operator(sub) = &self.data {
            for p in sub.iter() {
                result += p.sum_versions();
            }
        }

        result
    }

    fn calc(&self) -> u64 {
        use PacketData::*;

        match self.type_id {
            TypeId::Sum => {
                let mut total = 0;

                if let Operator(subpackets) = &self.data {
                    assert!(subpackets.len() > 0);

                    for p in subpackets.iter() {
                        total += p.calc();
                    }
                }

                total
            },
            TypeId::Prod => {
                let mut total = 1;

                if let Operator(subpackets) = &self.data {
                    assert!(subpackets.len() > 0);

                    for p in subpackets.iter() {
                        total *= p.calc();
                    }
                }

                total
            },
            TypeId::Min => {
                let mut min_value: u64 = u64::MAX;

                if let Operator(subpackets) = &self.data {
                    assert!(subpackets.len() > 0);

                    for p in subpackets.iter() {
                        let r = p.calc();

                        if r < min_value {
                            min_value = r;
                        }
                    }
                }

                min_value
            },
            TypeId::Max => {
                let mut max_value: u64 = 0;

                if let Operator(subpackets) = &self.data {
                    assert!(subpackets.len() > 0);

                    for p in subpackets.iter() {
                        let r = p.calc();

                        if r > max_value {
                            max_value = r;
                        }
                    }
                }

                max_value
            },
            TypeId::Lit => {
                if let Literal(value) = &self.data {
                    *value
                } else {
                    0
                }
            },
            TypeId::GreaterThan => {
                let mut result: u64 = 0;

                if let Operator(subpackets) = &self.data {
                    assert_eq!(subpackets.len(), 2);

                    let lhs = subpackets[0].calc();
                    let rhs = subpackets[1].calc();

                    if lhs > rhs {
                        result = 1;
                    }
                }

                result
            },
            TypeId::LessThan => {
                let mut result: u64 = 0;

                if let Operator(subpackets) = &self.data {
                    assert_eq!(subpackets.len(), 2);

                    let lhs = subpackets[0].calc();
                    let rhs = subpackets[1].calc();

                    if lhs < rhs {
                        result = 1;
                    }
                }

                result
            },
            TypeId::Equal => {
                let mut result: u64 = 0;

                if let Operator(subpackets) = &self.data {
                    assert_eq!(subpackets.len(), 2);

                    let lhs = subpackets[0].calc();
                    let rhs = subpackets[1].calc();

                    if lhs == rhs {
                        result = 1;
                    }
                }

                result
            },
        }
    }
}

#[allow(dead_code)]
fn part1(content: String) {
    let input = content
        .lines()
        .find(|line| {
            let r = line.trim();

            r.len() > 0 && !r.starts_with("#")
        });

    if let Some(value) = input {
        let p = Packet::new(value);

        println!("{:?}", p);

        println!("versions sum: {}", p.sum_versions());
    }
}

fn part2(content: String) {
    let input = content
        .lines()
        .find(|line| {
            let r = line.trim();

            r.len() > 0 && !r.starts_with("#")
        });

    if let Some(value) = input {
        let p = Packet::new(value);

        println!("{:?}", p);

        println!("calc: {}", p.calc());
        println!("versions sum: {}", p.sum_versions());
    }
}

fn main() {
    // let filename = "./inputs/day16-example.txt";
    let filename = "./inputs/day16.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not open file.");
    
    part2(content);
}

