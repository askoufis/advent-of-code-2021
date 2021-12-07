#[derive(Clone, Copy, PartialEq, Debug)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn flip(self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }

    fn to_char(self) -> char {
        match self {
            Bit::Zero => '0',
            Bit::One => '1',
        }
    }
}

type Bits = Vec<Bit>;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Bits> {
    let lines = input.lines();

    lines
        .map(str::chars)
        .map(|bit_string| {
            bit_string
                .map(|bit| if let '0' = bit { Bit::Zero } else { Bit::One })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(entries: &[Bits]) -> usize {
    let num_length = entries[0].len();
    let mut zero_count = Vec::with_capacity(num_length);
    for _ in 0..num_length {
        zero_count.push(0);
    }
    let mut one_count = Vec::with_capacity(num_length);
    for _ in 0..num_length {
        one_count.push(0);
    }

    entries.iter().for_each(|entry| {
        entry.iter().enumerate().for_each(|(index, bit)| match bit {
            Bit::Zero => zero_count[index] += 1,
            Bit::One => one_count[index] += 1,
        });
    });

    let mut gamma_bits = Vec::with_capacity(num_length);

    for i in 0..num_length {
        if zero_count[i] > one_count[i] {
            gamma_bits.push(Bit::Zero)
        } else {
            gamma_bits.push(Bit::One)
        }
    }

    let gamma_string: String = gamma_bits.clone().iter().map(|bit| bit.to_char()).collect();
    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();

    let epsilon_string: String = gamma_bits.iter().map(|bit| bit.flip().to_char()).collect();
    let epsilon = usize::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(entries: &[Bits]) -> usize {
    let num_bits = entries[0].len();

    let mut o2_bits = entries.to_vec();
    for i in 0..num_bits {
        if o2_bits.len() == 1 {
            break;
        }

        let mut zero_count = 0;
        let mut one_count = 0;
        o2_bits.iter().for_each(|entry| match entry[i] {
            Bit::Zero => zero_count += 1,
            Bit::One => one_count += 1,
        });
        if one_count >= zero_count {
            // keep entries with 1
            o2_bits = o2_bits
                .into_iter()
                .filter(|entry| entry[i] == Bit::One)
                .to_owned()
                .collect();
        } else {
            // keep entries with 0
            o2_bits = o2_bits
                .into_iter()
                .filter(|entry| entry[i] == Bit::Zero)
                .to_owned()
                .collect();
        }
    }
    let o2_rating_string: String = o2_bits[0].clone().iter().map(|bit| bit.to_char()).collect();
    let o2_rating = usize::from_str_radix(&o2_rating_string, 2).unwrap();

    let mut co2_bits = entries.to_vec();
    for i in 0..num_bits {
        if co2_bits.len() == 1 {
            break;
        }

        let mut zero_count = 0;
        let mut one_count = 0;
        co2_bits.iter().for_each(|entry| match entry[i] {
            Bit::Zero => zero_count += 1,
            Bit::One => one_count += 1,
        });
        if zero_count <= one_count {
            // keep entries with 0
            co2_bits = co2_bits
                .into_iter()
                .filter(|entry| entry[i] == Bit::Zero)
                .to_owned()
                .collect();
        } else {
            // keep entries with 1
            co2_bits = co2_bits
                .into_iter()
                .filter(|entry| entry[i] == Bit::One)
                .to_owned()
                .collect();
        }
    }

    let co2_rating_string: String = co2_bits[0]
        .clone()
        .iter()
        .map(|bit| bit.to_char())
        .collect();
    let co2_rating = usize::from_str_radix(&co2_rating_string, 2).unwrap();

    o2_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 198)
    }

    #[test]
    fn part2_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part2(&input), 230)
    }
}
