use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day05;

impl Day for Day05 {
    type ParsedData = (Vec<(i64, i64)>, Vec<i64>);

    type Part1Result = i64;

    type Part2Result = i64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day05.txt").expect("File Error"));

        let mut ranges = Vec::new();
        let mut ingredients = Vec::new();

        for line in reader.lines() {
            let l = line.unwrap();
            if l.contains("-") {
                let mut l = l.split("-");
                ranges.push((
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                ))
            } else if l.len() > 0 {
                ingredients.push(l.parse().unwrap())
            }
        }

        Ok((ranges, ingredients))
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let (ranges, ingredients) = input;
        ingredients
            .iter()
            .map(|i| {
                if ranges
                    .iter()
                    .map(|(s, e)| if i >= s && i <= e { 1 } else { 0 })
                    .sum::<i64>()
                    >= 1
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let (mut ranges, _) = input.clone();
        ranges.sort_by_key(|(s, _)| *s);

        loop {
            let mut new_ranges = Vec::new();
            let mut removed = HashSet::new();
            'outer: for (i, (r1s, r1e)) in ranges.iter().enumerate() {
                for j in (i + 1)..ranges.len() {
                    if removed.contains(&j) {
                        continue;
                    }

                    let (r2s, r2e) = ranges[j];

                    if r2s >= *r1s && r2e <= *r1e {
                        removed.insert(j);
                        new_ranges.push((*r1s, *r1e));
                        continue 'outer;
                    } else if r2s >= *r1s && *r1e >= r2s {
                        removed.insert(j);
                        new_ranges.push((*r1s, r2e));
                        continue 'outer;
                    }
                }

                if !removed.contains(&i) {
                    new_ranges.push((*r1s, *r1e));
                }
            }

            if ranges.len() == new_ranges.len() {
                ranges = new_ranges;
                break;
            }
            ranges = new_ranges;
        }

        ranges.iter().map(|(s, e)| e - s + 1).sum()
    }
}
