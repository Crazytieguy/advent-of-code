const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Vec<bool> {
    data.trim().chars().flat_map(hex_char_to_bits).collect()
}

fn hex_char_to_bits(c: char) -> Vec<bool> {
    let n = c.to_digit(16).unwrap();
    (0..4).rev().map(|i| (n & (1 << i)) != 0).collect()
}

fn bits_to_num(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| (b as usize) << i)
        .sum()
}

struct ParseOutcome {
    version_sum: usize,
    value: usize,
}

fn parse_bits(bits: &[bool], pos: &mut usize) -> ParseOutcome {
    let mut read_next = |size| {
        let out = &bits[*pos..*pos + size];
        *pos += size;
        out
    };

    let mut version_sum = bits_to_num(read_next(3));
    let type_id = bits_to_num(read_next(3));

    if type_id == 4 {
        let mut literal_bin = Vec::new();
        let mut keep_reading = true;
        while keep_reading {
            keep_reading = read_next(1)[0];
            literal_bin.extend_from_slice(read_next(4))
        }
        let value = bits_to_num(&literal_bin);
        return ParseOutcome { version_sum, value };
    }

    let len_type_is_subpackets = read_next(1)[0];
    let (num_subpackets, num_bits) = if len_type_is_subpackets {
        (bits_to_num(read_next(11)), usize::MAX)
    } else {
        (usize::MAX, bits_to_num(read_next(15)))
    };

    let initial_pos = *pos;
    let mut sub_values = Vec::new();
    while (*pos - initial_pos) < num_bits && sub_values.len() < num_subpackets {
        let parsed = parse_bits(bits, pos);
        version_sum += parsed.version_sum;
        sub_values.push(parsed.value);
    }

    let value = match type_id {
        0 => sub_values.into_iter().sum(),
        1 => sub_values.into_iter().product(),
        2 => sub_values.into_iter().min().unwrap(),
        3 => sub_values.into_iter().max().unwrap(),
        5 => (sub_values[0] > sub_values[1]) as usize,
        6 => (sub_values[0] < sub_values[1]) as usize,
        7 => (sub_values[0] == sub_values[1]) as usize,
        _ => panic!("{} is not a valid type id", type_id),
    };

    ParseOutcome { version_sum, value }
}

fn part_a(data: &'static str) -> usize {
    let bits = parse(data);
    parse_bits(&bits, &mut 0).version_sum
}

fn part_b(data: &'static str) -> usize {
    let bits = parse(data);
    parse_bits(&bits, &mut 0).value
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
