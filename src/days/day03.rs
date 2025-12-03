use super::Day;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Day03;

impl Day for Day03 {
    type ParsedData = Vec<String>;

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day03.txt").expect("File Error"));

        Ok(reader.lines().map(|l| l.unwrap()).collect())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        input.iter().fold(0, |a, s| {
            let chars = s.chars().collect::<Vec<char>>();
            let mut highest = 0;
            for i in 0..chars.len() {
                for j in (i + 1)..chars.len() {
                    let num = [chars[i], chars[j]]
                        .iter()
                        .copied()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    if num > highest {
                        highest = num;
                    }
                }
            }
            a + highest
        })
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        input.iter().fold(0, |a, s| {
            let chars = s.chars().collect::<Vec<char>>();

            let mut number: Vec<char> = Vec::new();

            let mut start = 0;
            let mut end = chars.len() - 12 + 1;

            while number.len() < 12 {
                let mut num_max = 0;
                let mut num_pos = 0;
                for (i, c) in chars[start..end].iter().enumerate() {
                    let d = c.to_digit(10).unwrap();
                    if d > num_max {
                        num_max = d;
                        num_pos = i;
                    }
                }
                number.push(chars[start + num_pos]);
                start += 1 + num_pos;
                end += 1;

                if end > chars.len() {
                    while number.len() < 12 {
                        number.push(chars[start]);
                        start += 1;
                    }
                    break;
                }
            }
            a + number.iter().collect::<String>().parse::<u64>().unwrap()
        })
    }
}
