use std::cmp::min;

use itertools::Itertools;

use aoc_macros::hashmap;

const PUZZLE_INPUT: &str = "0052E4A00905271049796FB8872A0D25B9FB746893847236200B4F0BCE5194401C9B9E3F9C63992C8931A65A1CCC0D222100511A00BCBA647D98BE29A397005E55064A9DFEEC86600BD002AF2343A91A1CCE773C26600D126B69D15A6793BFCE2775D9E4A9002AB86339B5F9AB411A15CCAF10055B3EFFC00BCCE730112FA6620076268CE5CDA1FCEB69005A3800D24F4DB66E53F074F811802729733E0040E5C5E5C5C8015F9613937B83F23B278724068018014A00588014005519801EC04B220116CC0402000EAEC03519801A402B30801A802138801400170A0046A800C10001AB37FD8EB805D1C266963E95A4D1A5FF9719FEF7FDB4FB2DB29008CD2BAFA3D005CD31EB4EF2EBE4F4235DF78C66009E80293AE9310D3FCBFBCA440144580273BAEE17E55B66508803C2E0087E630F72BCD5E71B32CCFBBE2800017A2C2803D272BCBCD12BD599BC874B939004B5400964AE84A6C1E7538004CD300623AC6C882600E4328F710CC01C82D1B228980292ECD600B48E0526E506F700760CCC468012E68402324F9668028200C41E8A30E00010D8B11E62F98029801AB88039116344340004323EC48873233E72A36402504CB75006EA00084C7B895198001098D91AE2190065933AA6EB41AD0042626A93135681A400804CB54C0318032200E47B8F71C0001098810D61D8002111B228468000E5269324AD1ECF7C519B86309F35A46200A1660A280150968A4CB45365A03F3DDBAE980233407E00A80021719A1B4181006E1547D87C6008E0043337EC434C32BDE487A4AE08800D34BC3DEA974F35C20100BE723F1197F59E662FDB45824AA1D2DDCDFA2D29EBB69005072E5F2EDF3C0B244F30E0600AE00203229D229B342CC007EC95F5D6E200202615D000FB92CE7A7A402354EE0DAC0141007E20C5E87A200F4318EB0C";

enum Packet {
    Literal { version_id: usize, value: usize },
    Operator { version_id: usize, type_id: usize, packets: Vec<Packet> },
}


fn bits_to_usize(value: &[usize]) -> usize {
    value.iter()
         .rev()
         .enumerate()
         .fold(0, |acc, (i, b)| {
             acc + b * 2_usize.pow(i as u32)
         })
}

impl Packet {
    fn from_bits(bits: &[usize]) -> (Self, usize) {
        let version_id = bits_to_usize(&bits[0..3]);
        let type_id = bits_to_usize(&bits[3..6]);

        match type_id {
            4 => {
                let mut packets = vec![];
                let mut offset = 0;

                for i in (6..bits.len()).step_by(5) {
                    let mut subbits = bits[i + 1..i + 5].to_vec();
                    while subbits.len() < 4 {
                        subbits.push(0);
                    }
                    packets.extend(subbits);

                    if bits[i] == 0 {
                        offset = min(i + 5, bits.len());
                        break;
                    }
                }

                (Packet::Literal { version_id, value: bits_to_usize(&packets) }, offset)
            }
            _ => {
                let length_type_id = bits[6];
                let mut packets = vec![];

                let offset = match length_type_id {
                    0 => {
                        let mut index = 7 + 15;
                        let size = bits_to_usize(&bits[7..index]);

                        let end = index + size;
                        while index < end {
                            let (packet, off) = Packet::from_bits(&bits[index..end]);
                            packets.push(packet);
                            index += off;
                        }

                        end
                    }
                    1 => {
                        let mut index = 7 + 11;
                        let num_packets = bits_to_usize(&bits[7..index]);
                        for _ in 0..num_packets {
                            let (packet, off) = Packet::from_bits(&bits[index..]);
                            packets.push(packet);
                            index += off;
                        }
                        index
                    }
                    _ => panic!("Invalid length_type_id: {}", length_type_id)
                };

                (Packet::Operator { version_id, type_id, packets }, offset)
            }
        }
    }

    fn version_sum(&self) -> usize {
        let v = match self {
            Packet::Literal { version_id, .. } => *version_id,
            Packet::Operator { version_id, packets, .. } => {
                *version_id + packets.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        };

        v
    }

    fn get_value(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { type_id, packets, .. } => {
                match type_id {
                    0 => packets.iter().map(|p| p.get_value()).sum::<usize>(),
                    1 => packets.iter().fold(1, |acc, p| acc * p.get_value()),
                    2 => packets.iter().map(|p| p.get_value()).min().unwrap(),
                    3 => packets.iter().map(|p| p.get_value()).max().unwrap(),
                    _ => {
                        let p1 = &packets[0].get_value();
                        let p2 = &packets[1].get_value();

                        match type_id {
                            5 => if p1 > p2 { 1 } else { 0 },
                            6 => if p1 < p2 { 1 } else { 0 },
                            7 => if p1 == p2 { 1 } else { 0 },
                            _ => panic!("Invalid type_id: {}", type_id)
                        }
                    }
                }
            }
        }
    }
}

fn parse_packet(input: &str) -> Packet {
    let map = hashmap![
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
    ];

    let bits = input.chars()
                    .map(|c| map[&c])
                    .join("")
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect_vec();


    Packet::from_bits(&bits[..]).0
}

pub fn solve_a() {
    let packet = parse_packet(PUZZLE_INPUT);
    let ans = packet.version_sum();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let packet = parse_packet(PUZZLE_INPUT);
    let ans = packet.get_value();
    
    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::parse_packet;

    #[test]
    fn test_version_sum() {
        for (inp, exp) in [
            ("D2FE28", 6),
            ("38006F45291200", 1 + 6 + 2),
            ("EE00D40C823060", 7 + 2 + 4 + 1),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            let packet = parse_packet(inp);
            assert_eq!(packet.version_sum(), exp, "Invalid version sum for '{}'", inp);
        }
    }

    #[test]
    fn test_get_value() {
        for (inp, exp) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            let packet = parse_packet(inp);
            assert_eq!(packet.get_value(), exp, "Invalid value for '{}'", inp);
        }
    }
}
