use rand::{thread_rng, Rng};

use crate::spn::SPN;

#[allow(dead_code)]
mod spn;

fn main() {
    let table = analyze_s(SPN::S);
    print_s_analysis(table);
    let ciphers = statistics();
    let key = search_key(vec![0b010000001, 0b011000101], &ciphers);
    println!("found key = {:09b}", key);
}

fn analyze_s(s: [u16; 8]) -> Vec<Vec<u8>> {
    (1..=7)
        .map(|i| {
            (1..=7)
                .map(|j| {
                    s.iter()
                        .enumerate()
                        .map(|(input, output)| {
                            let masked_input = (input & i).count_ones() % 2;
                            let masked_output = (output & j).count_ones() % 2;
                            (masked_input == masked_output) as u8
                        })
                        .sum::<u8>()
                })
                .collect()
        })
        .collect()
}

fn print_s_analysis(table: Vec<Vec<u8>>) {
    println!(" | 1 2 3 4 5 6 7");
    println!("----------------");
    table.iter().enumerate().for_each(|(i, row)| {
        print!("{}|", i + 1);
        row.iter().for_each(|value| match value {
            0 => print!(" \x1b[31m{}\x1b[0m", value),
            8 => print!(" \x1b[32m{}\x1b[0m", value),
            _ => print!(" {}", value),
        });
        println!();
    });
}

fn get(num: u16, i: usize) -> u16 {
    (num >> (9 - i)) & 0x1
}

fn statistics() -> Vec<(u16, u16)> {
    let spn = SPN::new(3);
    // let key: u16 = thread_rng().gen_range(0..=0x1ff);
    let key: u16 = 0b011000101;
    println!("key = {:09b}", key);

    let mut stats = vec![0; 9];
    let mut ciphers: Vec<(u16, u16)> = Vec::new();
    for _ in 0..100 {
        let x = thread_rng().gen_range(0..=0x1ff);
        let y = spn.encode(x, key);
        ciphers.push((x, y));
        stats[0] += get(y, 5) ^ get(y, 8) ^ get(x, 2);
        stats[1] += get(y, 4) ^ get(y, 7) ^ get(x, 1) ^ get(x, 2);
        stats[2] += get(y, 4) ^ get(y, 7) ^ get(y, 9) ^ get(x, 1) ^ get(x, 2);
        stats[3] += get(y, 4) ^ get(y, 7) ^ get(x, 1);
        stats[4] += get(y, 4) ^ get(y, 5) ^ get(y, 7) ^ get(x, 1);
        stats[5] += get(y, 4) ^ get(y, 5) ^ get(y, 6) ^ get(y, 7) ^ get(x, 1);
        stats[6] += get(y, 2) ^ get(y, 3) ^ get(x, 9);
        stats[7] += get(y, 7) ^ get(x, 1) ^ get(x, 2) ^ get(x, 4) ^ get(x, 5);
        stats[8] += get(y, 5) ^ get(y, 6) ^ get(y, 8) ^ get(y, 9) ^ get(x, 3);
    }
    stats
        .iter()
        .enumerate()
        .for_each(|(i, &stat)| println!("stat[{}] = {}", i, stat as f32 / 100.));
    ciphers
}

fn check_key(ciphers: &Vec<(u16, u16)>, key: u16) -> bool {
    let spn = SPN::new(3);
    ciphers
        .iter()
        .map(|&(x, y)| spn.encode(x, key) == y)
        .all(|ok| ok)
}

fn search_key(keys: Vec<u16>, ciphers: &Vec<(u16, u16)>) -> u16 {
    *keys
        .iter()
        .find(|&&key| check_key(ciphers, key))
        .expect("key not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_spn() {
        for _ in 0..100 {
            let spn = SPN::new(3);
            let x: u16 = thread_rng().gen_range(0..=0x1ff);
            let key: u16 = thread_rng().gen_range(0..=0x1ff);
            let y = spn.encode(x, key);
            assert_eq!(spn.decode(y, key), x)
        }
    }
}
