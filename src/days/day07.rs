use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Coords {
    x: usize,
    y: usize,
}

pub struct Day07;

impl Day for Day07 {
    type ParsedData = (Coords, Vec<Coords>, Coords);

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day07.txt").expect("File Error"));

        let mut start = None;
        let mut splitters = Vec::new();
        let mut num_lines = 0;
        let mut num_chars = 0;

        reader.lines().enumerate().for_each(|(i, l)| {
            num_chars = l.as_ref().unwrap().len();
            l.unwrap().chars().enumerate().for_each(|(j, c)| match c {
                'S' => start = Some(Coords { x: j, y: i }),
                '^' => splitters.push(Coords { x: j, y: i }),
                _ => (),
            });
            num_lines += 1;
        });

        Ok((
            start.unwrap(),
            splitters,
            Coords {
                x: num_chars,
                y: num_lines,
            },
        ))
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let (start, splitters, dimensions) = input;

        let mut visited: HashSet<Coords> = HashSet::new();
        let mut splitters_hit: HashSet<Coords> = HashSet::new();

        let mut queue: Vec<Coords> = Vec::new();
        queue.push(*start);

        while queue.len() > 0 {
            let Coords { x, mut y } = queue.pop().unwrap();

            y += 1;

            if y >= dimensions.y || visited.contains(&Coords { x, y }) {
                continue;
            }

            visited.insert(Coords { x, y });

            if splitters.contains(&Coords { x, y }) {
                if x > 0 {
                    queue.push(Coords { x: x - 1, y });
                }
                if x < dimensions.x - 1 {
                    queue.push(Coords { x: x + 1, y });
                }
                splitters_hit.insert(Coords { x, y });
            } else {
                queue.push(Coords { x, y })
            }
        }

        splitters_hit.len() as u64
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let (start, splitters, dimensions) = input;
        let lookup: HashMap<Coords, u64> = HashMap::new();
        *part2_helper(
            get_next_splitter(*start, &splitters, *dimensions).unwrap(),
            splitters,
            *dimensions,
            lookup,
        )
        .get(&get_next_splitter(*start, &splitters, *dimensions).unwrap())
        .unwrap()
    }
}

fn part2_helper(
    coords: Coords,
    splitters: &Vec<Coords>,
    dimensions: Coords,
    mut lookup: HashMap<Coords, u64>,
) -> HashMap<Coords, u64> {
    let Coords { x, y } = coords;

    let left_splitter = if x > 0 {
        if let Some(next_splitter) =
            get_next_splitter(Coords { x: x - 1, y }, splitters, dimensions)
        {
            if !lookup.contains_key(&next_splitter) {
                lookup = part2_helper(next_splitter, splitters, dimensions, lookup)
            }
            Some(next_splitter)
        } else {
            None
        }
    } else {
        None
    };

    let right_splitter = if x < dimensions.x {
        if let Some(next_splitter) =
            get_next_splitter(Coords { x: x + 1, y }, splitters, dimensions)
        {
            if !lookup.contains_key(&next_splitter) {
                lookup = part2_helper(next_splitter, splitters, dimensions, lookup)
            }
            Some(next_splitter)
        } else {
            None
        }
    } else {
        None
    };

    let left = if let Some(splitter) = left_splitter {
        *lookup.get(&splitter).unwrap()
    } else {
        1
    };

    let right = if let Some(splitter) = right_splitter {
        *lookup.get(&splitter).unwrap()
    } else {
        1
    };

    lookup.insert(Coords { x, y }, left + right);
    lookup
}

fn get_next_splitter(
    coords: Coords,
    splitters: &Vec<Coords>,
    dimensions: Coords,
) -> Option<Coords> {
    let Coords { x, mut y } = coords;

    loop {
        if splitters.contains(&Coords { x, y }) {
            return Some(Coords { x, y });
        }
        y += 1;
        if y >= dimensions.y {
            return None;
        }
    }
}
