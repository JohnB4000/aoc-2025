use std::{fs::File, io::BufReader};

use super::Day;

pub struct Day06;

impl Day for Day06 {
    type ParsedData = (Vec<i32>, Vec<i32>);

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day01.txt").expect("File Error"));
        todo!()
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        0
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        0
    }
}
