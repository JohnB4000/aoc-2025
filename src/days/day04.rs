use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day04;

impl Day for Day04 {
    type ParsedData = Vec<String>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day04.txt").expect("File Error"));
        Ok(reader.lines().map(|l| l.unwrap()).collect())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let offsets = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (0, 1),
            (1, 1),
            (1, 0),
            (-1, 1),
        ];
        input
            .iter()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => 0,
                        '@' => {
                            if offsets
                                .iter()
                                .map(|(o1, o2)| {
                                    let (pos_x, pos_y) = (i as i32 + o1, j as i32 + o2);
                                    if pos_x < 0
                                        || pos_y < 0
                                        || pos_x >= input.len() as i32
                                        || pos_y >= l.len() as i32
                                    {
                                        0
                                    } else {
                                        if input[pos_x as usize].chars().collect::<Vec<char>>()
                                            [pos_y as usize]
                                            == '@'
                                        {
                                            1
                                        } else {
                                            0
                                        }
                                    }
                                })
                                .sum::<i32>()
                                < 4
                            {
                                1
                            } else {
                                0
                            }
                        }
                        _ => todo!(),
                    })
                    .sum::<i32>()
            })
            .sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut input = input.clone();
        let offsets = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (0, 1),
            (1, 1),
            (1, 0),
            (-1, 1),
        ];
        let mut count = 0;

        loop {
            let start_count = count.clone();

            let mut indexes = Vec::new();
            for i in 0..input.len() {
                let chars: Vec<char> = input[i].chars().collect();
                for j in 0..chars.len() {
                    match chars[j] {
                        '.' => continue,
                        '@' => {
                            if offsets
                                .iter()
                                .map(|(o1, o2)| {
                                    let (pos_x, pos_y) = (i as i32 + o1, j as i32 + o2);
                                    if pos_x < 0
                                        || pos_y < 0
                                        || pos_x >= input.len() as i32
                                        || pos_y >= chars.len() as i32
                                    {
                                        0
                                    } else {
                                        if input[pos_x as usize].chars().collect::<Vec<char>>()
                                            [pos_y as usize]
                                            == '@'
                                        {
                                            1
                                        } else {
                                            0
                                        }
                                    }
                                })
                                .sum::<i32>()
                                < 4
                            {
                                indexes.push((i, j))
                            } else {
                                continue;
                            }
                        }
                        _ => todo!(),
                    };
                }
            }

            count += indexes.len();

            if count == start_count {
                break;
            }

            for (x, y) in indexes {
                let mut new_row: Vec<char> = input[x].chars().clone().collect();
                new_row[y] = '.';
                input[x] = new_row.iter().collect();
            }
        }
        count as i32
    }
}
