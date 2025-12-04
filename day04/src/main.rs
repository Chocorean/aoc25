use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

#[derive(Clone, Debug)]
pub struct Map {
    map: HashMap<(usize, usize), bool>,
    pub x: usize,
    pub y: usize,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let x = input.lines().collect::<Vec<_>>().len();
        let y = input.lines().collect::<Vec<_>>().first().unwrap().len();
        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                map.insert((i, j), char == '@');
            }
        }
        Self { map, x, y }
    }

    pub fn neighbors(&self, pos: (usize, usize)) -> usize {
        let mut nb = 0;
        for di in -1..2 {
            for dj in -1..2 {
                // ignoring self
                if di == 0 && dj == 0 {
                    continue;
                }

                let i = di + pos.0 as isize;
                let j = dj + pos.1 as isize;

                // negative indexes
                if i < 0 || j < 0 {
                    continue;
                }

                let opt_v = self.map.get(&(i as usize, j as usize));
                if let Some(v) = opt_v {
                    if *v {
                        nb += 1;
                    }
                }
            }
        }
        nb
    }

    pub fn paper(&self, pos: (usize, usize)) -> bool {
        *self.map.get(&pos).unwrap_or(&false)
    }

    pub fn remove(&mut self, pos: (usize, usize)) {
        self.map.insert(pos, false);
    }
}

pub fn task1(input: &str) -> u64 {
    let map = Map::new(&input);
    let mut sum = 0;
    for i in 0..map.x {
        for j in 0..map.y {
            if map.neighbors((i, j)) < 4 && map.paper((i, j)) {
                sum += 1;
            }
        }
    }
    sum
}

pub fn task2(input: &str) -> u64 {
    let mut map = Map::new(&input);
    let mut sum = 0;
    let mut old_sum = 1;
    while sum != old_sum {
        old_sum = sum;
        for i in 0..map.x {
            for j in 0..map.y {
                if map.neighbors((i, j)) < 4 && map.paper((i, j)) {
                    sum += 1;
                    map.remove((i, j));
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let input = include_str!("../example.txt");
        assert_eq!(task1(&input), 13);
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example.txt");
        assert_eq!(task2(&input), 43);
    }
}
