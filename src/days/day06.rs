use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Multiply,
}

pub struct Day06;

impl Day for Day06 {
    type ParsedData = Vec<String>;

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day06.txt").expect("File Error"));

        Ok(reader.lines().map(|l| l.unwrap()).collect())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut rows: Vec<Vec<u64>> = Vec::new();
        let mut operations = Vec::new();

        input.iter().for_each(|l| {
            let mut l = l.split(" ").peekable();
            match l.peek() {
                Some(&"*") | Some(&"+") => {
                    operations = l
                        .filter_map(|o| match o {
                            "*" => Some(Operation::Multiply),
                            "+" => Some(Operation::Add),
                            _ => None,
                        })
                        .collect()
                }
                Some(_) => rows.push(
                    l.filter_map(|o| {
                        if o.len() != 0 {
                            Some(o.parse().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect(),
                ),
                None => todo!(),
            }
        });

        let mut data = Vec::new();
        for i in 0..rows[0].len() {
            let mut entry = Vec::new();
            for j in 0..rows.len() {
                entry.push(rows[j][i]);
            }
            data.push((entry, operations[i]))
        }

        data.iter()
            .map(|(values, operation)| match operation {
                Operation::Add => values.iter().sum::<u64>(),
                Operation::Multiply => values.iter().fold(1, |a, v| a * v),
            })
            .sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut data: Vec<(Vec<u64>, Operation)> = Vec::new();

        let mut entry: Vec<u64> = Vec::new();
        let mut operation = Operation::Multiply;
        input[input.len() - 1]
            .chars()
            .enumerate()
            .for_each(|(i, s)| {
                if s == '*' {
                    data.push((entry.clone(), operation));
                    entry = Vec::new();
                    operation = Operation::Multiply
                } else if s == '+' {
                    data.push((entry.clone(), operation));
                    entry = Vec::new();
                    operation = Operation::Add
                }

                let number = (0..(input.len() - 1))
                    .filter_map(|j| {
                        let c = input[j].chars().collect::<Vec<char>>()[i];
                        if c != ' ' { Some(c) } else { None }
                    })
                    .collect::<String>();

                if number.len() > 0 {
                    entry.push(number.parse().unwrap())
                }
            });
        data.push((entry.clone(), operation));

        data.remove(0);

        data.iter()
            .map(|(values, operation)| match operation {
                Operation::Add => values.iter().sum::<u64>(),
                Operation::Multiply => values.iter().fold(1, |a, v| a * v),
            })
            .sum()
    }
}
