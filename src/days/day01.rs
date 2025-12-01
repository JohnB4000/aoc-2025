use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day01;

impl Day for Day01 {
    type ParsedData = Vec<(char, i32)>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day01.txt").expect("File Error"));

        Ok(reader
            .lines()
            .map(|l| match l {
                Ok(s) => {
                    let mut i = s.chars();
                    (
                        i.next().unwrap(),
                        i.collect::<String>().parse::<i32>().unwrap(),
                    )
                }
                Err(_) => todo!(),
            })
            .collect::<Vec<(char, i32)>>())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut count = 0;
        let mut position = 50;

        for (direction, magnitude) in input {
            match direction {
                'L' => position = ((position - magnitude) + 100) % 100,
                'R' => position = ((position + magnitude) + 100) % 100,
                _ => panic!("Invalid direction"),
            }
            if position == 0 {
                count += 1;
            }
        }

        count
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut count = 0;

        let mut position = 50;

        for (direction, magnitude) in input {
            let (new_position, new_count) = click(position, *magnitude, *direction);
            count += new_count;
            position = new_position;
        }

        count
    }
}

fn click(mut position: i32, mut clicks: i32, direction: char) -> (i32, i32) {
    let mut times = 0;

    while clicks > 0 {
        match direction {
            'L' => position = ((position - 1) + 100) % 100,
            'R' => position = ((position + 1) + 100) % 100,
            _ => panic!("Invalid direction"),
        }
        if position == 0 {
            times += 1;
        }
        clicks -= 1;
    }

    (position, times)
}
