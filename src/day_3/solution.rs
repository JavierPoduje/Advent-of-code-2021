use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let binaries = read_one_per_line::<String>("./src/day_3/input.txt").unwrap();
    (part1(&binaries) as u64, part2(&binaries) as u64)
}

fn part2(binaries: &Vec<String>) -> i32 {
    let cols: usize = binaries[0].len();

    let bytes: Vec<Vec<Bit>> = binaries
        .clone()
        .into_iter()
        .filter(|binary| binary != "")
        .map(|binary| get_bits(binary))
        .collect();

    let oxygen_renerator_rating = parse_int(calculate_oxygen_renerator_rating(
        bytes.clone(),
        &bytes,
        &cols,
    ));
    let c02_scrubber_rating =
        parse_int(calculate_c02_scrubber_rating(bytes.clone(), &bytes, &cols));

    oxygen_renerator_rating * c02_scrubber_rating
}

fn calculate_c02_scrubber_rating(
    mut remaining: Vec<Vec<Bit>>,
    bytes: &Vec<Vec<Bit>>,
    length: &usize,
) -> Vec<Bit> {
    let mut idx = 0;
    let mut bit_by_col_idx: Vec<ColumnBit> = bits_by_col_idx(&bytes, &length.clone());
    loop {
        let bits = &bit_by_col_idx[idx];

        if bits.ones < bits.zeroes {
            remaining.retain(|byte| {
                if byte.len() == 0 {
                    return false;
                }
                match byte[idx] {
                    Bit::One => true,
                    Bit::Zero => false,
                }
            })
        } else {
            remaining.retain(|byte| {
                if byte.len() == 0 {
                    return false;
                }
                match byte[idx] {
                    Bit::One => false,
                    Bit::Zero => true,
                }
            })
        }

        idx += 1;

        if remaining.len() == 1 {
            return remaining[0].clone();
        }

        bit_by_col_idx = bits_by_col_idx(&remaining, &length.clone());
    }
}

fn calculate_oxygen_renerator_rating(
    mut remaining: Vec<Vec<Bit>>,
    bytes: &Vec<Vec<Bit>>,
    length: &usize,
) -> Vec<Bit> {
    let mut idx = 0;
    let mut bit_by_col_idx: Vec<ColumnBit> = bits_by_col_idx(&bytes, &length.clone());
    loop {
        let bits = &bit_by_col_idx[idx];
        if bits.zeroes > bits.ones {
            remaining.retain(|byte| {
                if byte.len() == 0 {
                    return false;
                }

                match byte[idx] {
                    Bit::Zero => true,
                    Bit::One => false,
                }
            })
        } else {
            remaining.retain(|byte| {
                if byte.len() == 0 {
                    return false;
                }

                match byte[idx] {
                    Bit::Zero => false,
                    Bit::One => true,
                }
            })
        }

        idx += 1;

        if remaining.len() == 1 {
            return remaining[0].clone();
        }

        bit_by_col_idx = bits_by_col_idx(&remaining, &length.clone());
    }
}

fn part1(binaries: &Vec<String>) -> i32 {
    let cols: usize = binaries[0].len();
    let mut bit_by_col_idx: Vec<ColumnBit> = vec![ColumnBit::new(); cols];

    for (_bin_idx, binary) in binaries.into_iter().enumerate() {
        let bits = get_bits(binary.clone());
        for (idx, bit) in bits.into_iter().enumerate() {
            match bit {
                Bit::Zero => bit_by_col_idx[idx].zeroes += 1,
                Bit::One => bit_by_col_idx[idx].ones += 1,
            }
        }
    }

    let gamma = parse_int(gamma_byte(&bit_by_col_idx));
    let epsilon = parse_int(epsilon_byte(&bit_by_col_idx));

    gamma * epsilon
}

fn parse_int(bites: Vec<Bit>) -> i32 {
    bites.into_iter().fold(0, |acc, bite| {
        let bit = match bite {
            Bit::Zero => 0,
            Bit::One => 1,
        };
        (acc << 1) + bit
    })
}

fn get_bits(str: String) -> Vec<Bit> {
    str.chars().into_iter().fold(vec![], |mut bits, bit| {
        if bit == '0' {
            bits.push(Bit::Zero)
        } else {
            bits.push(Bit::One)
        }
        bits
    })
}

fn gamma_byte(bites: &Vec<ColumnBit>) -> Vec<Bit> {
    bites.into_iter().fold(vec![], |mut byte, column_bit| {
        if column_bit.ones >= column_bit.zeroes {
            byte.push(Bit::One)
        } else {
            byte.push(Bit::Zero)
        }
        byte
    })
}

fn epsilon_byte(bites: &Vec<ColumnBit>) -> Vec<Bit> {
    bites.into_iter().fold(vec![], |mut byte, column_bit| {
        if column_bit.ones <= column_bit.zeroes {
            byte.push(Bit::One)
        } else {
            byte.push(Bit::Zero)
        }
        byte
    })
}

fn bits_by_col_idx(bytes: &Vec<Vec<Bit>>, length: &usize) -> Vec<ColumnBit> {
    let mut bit_by_col_idx: Vec<ColumnBit> = vec![ColumnBit::new(); length.clone()];
    for byte in bytes.into_iter() {
        for (idx, bit) in byte.into_iter().enumerate() {
            match bit {
                Bit::Zero => bit_by_col_idx[idx].zeroes += 1,
                Bit::One => bit_by_col_idx[idx].ones += 1,
            }
        }
    }
    bit_by_col_idx
}

#[derive(Clone, Debug)]
enum Bit {
    Zero,
    One,
}

#[derive(Clone, Debug)]
struct ColumnBit {
    ones: u64,
    zeroes: u64,
}

impl ColumnBit {
    pub fn new() -> Self {
        Self { ones: 0, zeroes: 0 }
    }
}
