use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day09;

impl Day for Day09 {
    type ParsedData = Vec<(i64, i64)>;

    type Part1Result = i64;

    type Part2Result = i64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day09.txt").expect("File Error"));
        Ok(reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut l = l.split(",");
                (
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                )
            })
            .collect())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        input
            .iter()
            .enumerate()
            .map(|(i, (x1, y1))| {
                let mut max = 0;
                for j in (i + 1)..input.len() {
                    let (x2, y2) = input[j];
                    let square = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
                    if square > max {
                        max = square;
                    }
                }
                max
            })
            .max()
            .unwrap()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        0
    }
}
