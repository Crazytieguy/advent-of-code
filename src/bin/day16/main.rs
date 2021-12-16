#![feature(array_chunks)]

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Vec<bool> {
    data.trim().chars().flat_map(hex_char_to_bits).collect()
}

fn hex_char_to_bits(c: char) -> [bool; 4] {
    const F: bool = false;
    const T: bool = true;

    match c {
        '0' => [F, F, F, F],
        '1' => [F, F, F, T],
        '2' => [F, F, T, F],
        '3' => [F, F, T, T],
        '4' => [F, T, F, F],
        '5' => [F, T, F, T],
        '6' => [F, T, T, F],
        '7' => [F, T, T, T],
        '8' => [T, F, F, F],
        '9' => [T, F, F, T],
        'A' => [T, F, T, F],
        'B' => [T, F, T, T],
        'C' => [T, T, F, F],
        'D' => [T, T, F, T],
        'E' => [T, T, T, F],
        'F' => [T, T, T, T],
        _ => panic!(),
    }
}

fn bin_to_num(bin: &[bool]) -> usize {
    bin.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| (b as usize) << i)
        .sum()
}

struct ParseResult {
    version_sum: usize,
    num_bits_read: usize,
    value: usize,
}

fn parse_bits(bits: &[bool]) -> ParseResult {
    let mut version_sum = bin_to_num(&bits[0..3]);
    let t = bin_to_num(&bits[3..6]);
    if t == 4 {
        let mut num_bits_read = 6;
        let mut literal_bin = Vec::new();
        for [keep_reading, val @ ..] in bits[6..].array_chunks::<5>() {
            literal_bin.extend_from_slice(val);
            num_bits_read += 5;
            if !keep_reading {
                break;
            }
        }
        let value = bin_to_num(&literal_bin);
        ParseResult {
            version_sum,
            num_bits_read,
            value,
        }
    } else {
        let len_type_is_subpackets = bits[6];
        let mut num_bits_read = 7;
        let mut sub_values = Vec::new();
        if len_type_is_subpackets {
            let num_subpackets = bin_to_num(&bits[7..7 + 11]) as usize;
            num_bits_read += 11;
            for _ in 0..num_subpackets {
                let parsed = parse_bits(&bits[num_bits_read..]);
                num_bits_read += parsed.num_bits_read;
                version_sum += parsed.version_sum;
                sub_values.push(parsed.value);
            }
        } else {
            let num_bits = bin_to_num(&bits[7..7 + 15]) as usize;
            if num_bits > bits.len() {
                panic!("too many bits!")
            }
            num_bits_read += 15;
            let mut data_bits_read = 0;
            while data_bits_read < num_bits {
                let parsed = parse_bits(&bits[num_bits_read..]);
                num_bits_read += parsed.num_bits_read;
                data_bits_read += parsed.num_bits_read;
                version_sum += parsed.version_sum;
                sub_values.push(parsed.value);
            }
            assert_eq!(num_bits, data_bits_read);
            assert_eq!(num_bits_read, num_bits + 22);
        }
        if t > 4 {
            assert_eq!(sub_values.len(), 2);
        }
        let value = match t {
            0 => sub_values.into_iter().sum(),
            1 => sub_values.into_iter().product(),
            2 => sub_values.into_iter().min().unwrap(),
            3 => sub_values.into_iter().max().unwrap(),
            5 => (sub_values[0] > sub_values[1]) as usize,
            6 => (sub_values[0] < sub_values[1]) as usize,
            7 => (sub_values[0] == sub_values[1]) as usize,
            _ => panic!("{} is not a valid type id", t),
        };
        ParseResult {
            version_sum,
            num_bits_read,
            value,
        }
    }
}

fn part_a(data: &'static str) -> usize {
    let bits = parse(data);
    parse_bits(&bits).version_sum
}

fn part_b(data: &'static str) -> usize {
    let bits = parse(data);
    parse_bits(&bits).value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        assert_eq!(part_a("8A004A801A8002F478"), 16);
        assert_eq!(part_a("620080001611562C8802118E34"), 12);
        assert_eq!(part_a("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_a("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b("C200B40A82"), 3);
        assert_eq!(part_b("04005AC33890"), 54);
        assert_eq!(part_b("880086C3E88112"), 7);
        assert_eq!(part_b("CE00C43D881120"), 9);
        assert_eq!(part_b("D8005AC2A8F0"), 1);
        assert_eq!(part_b("F600BC2D8F"), 0);
        assert_eq!(part_b("9C005AC2F8F0"), 0);
        assert_eq!(part_b("9C0141080250320F1802104A08"), 1);
    }
}
