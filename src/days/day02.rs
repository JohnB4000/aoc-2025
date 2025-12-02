use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day02;

impl Day for Day02 {
    type ParsedData = Vec<(u64, u64)>;

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day02.txt").expect("File Error"));
        let ranges: Self::ParsedData = Vec::new();

        Ok(reader
            .lines()
            .map(|l| l.unwrap())
            .flat_map(|l| {
                l.split(",")
                    .map(|range| {
                        let mut range = range.split("-");
                        (
                            range.next().unwrap().parse::<u64>().unwrap(),
                            range.next().unwrap().parse::<u64>().unwrap(),
                        )
                    })
                    .collect::<Vec<(u64, u64)>>()
            })
            .collect())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        input.iter().fold(0u64, |a, (r1, r2)| {
            a + (*r1..=*r2).fold(0u64, |c, n| if is_repeat_p1(n) { c + n } else { c })
        })
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        input.iter().fold(0u64, |a, (r1, r2)| {
            a + (*r1..=*r2).fold(0u64, |c, n| if is_repeat_p2(n) { c + n } else { c })
        })
    }
}

fn is_repeat_p1(n: u64) -> bool {
    let n = n.to_string();

    if n.len() % 2 != 0 {
        return false;
    }

    let (first, last) = n.split_at(n.len() / 2);

    first == last
}

fn is_repeat_p2(n: u64) -> bool {
    let n = n.to_string();

    'outer: for size in 1..n.len() {
        let (pattern, mut rest) = n.split_at(size);

        loop {
            if size > rest.len() {
                continue 'outer;
            }

            let (search, new_rest) = rest.split_at(size);
            rest = new_rest;

            if pattern != search {
                if rest.len() == 0 {
                    return false;
                }
                continue 'outer;
            }
            if rest.len() == 0 {
                return true;
            }
        }
    }

    false
}
