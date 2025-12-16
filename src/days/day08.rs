use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct UnionFind {
    representitives: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            representitives: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn find(&self, index: usize) -> usize {
        if self.representitives[index] == index {
            return index;
        }
        self.find(self.representitives[index])
    }

    pub fn union(&mut self, index1: usize, index2: usize) {
        let index1 = self.find(index1);
        let index2 = self.find(index2);
        self.representitives[index1] = index2;
        self.size[index2] += self.size[index1];
    }
}

pub struct Day08;

impl Day for Day08 {
    type ParsedData = Vec<(i64, i64, i64)>;

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day08.txt").expect("File Error"));
        Ok(reader
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut l = l.split(",");
                (
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                )
            })
            .collect())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut vertices: Vec<(usize, usize, f32)> = Vec::new();
        for (i, (x1, y1, z1)) in input.iter().enumerate() {
            for (j, (x2, y2, z2)) in input[(i + 1)..].iter().enumerate() {
                vertices.push((
                    i,
                    i + j + 1,
                    (((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1))
                        as f32)
                        .sqrt(),
                ))
            }
        }

        vertices.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

        let mut uf = UnionFind::new(input.len());

        let mut count = 1;
        vertices.iter().for_each(|(a, b, size)| {
            if uf.find(*a) != uf.find(*b) && count <= 1000 {
                uf.union(*a, *b);
            }
            count += 1;
        });

        let mut visited = HashSet::new();
        let mut sizes = Vec::new();
        for v in &uf.representitives {
            let root = uf.find(*v);
            if !visited.contains(&root) {
                sizes.push(uf.size[root] as u64);
                visited.insert(root);
            }
        }

        sizes.sort();
        sizes.reverse();

        sizes[0] * sizes[1] * sizes[2]
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut vertices: Vec<(usize, usize, f32)> = Vec::new();
        for (i, (x1, y1, z1)) in input.iter().enumerate() {
            for (j, (x2, y2, z2)) in input[(i + 1)..].iter().enumerate() {
                vertices.push((
                    i,
                    i + j + 1,
                    (((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1))
                        as f32)
                        .sqrt(),
                ))
            }
        }

        vertices.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

        let mut uf = UnionFind::new(input.len());

        let mut last_connection = None;
        vertices.iter().for_each(|(a, b, size)| {
            if uf.find(*a) != uf.find(*b) {
                uf.union(*a, *b);
                last_connection = Some((*a, *b));
            }
        });

        let (a, b) = last_connection.unwrap();
        let (x1, _, _) = input[a];
        let (x2, _, _) = input[b];

        (x1 * x2) as u64
    }
}
