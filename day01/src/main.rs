fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

pub struct Dial(u32);

impl Dial {
    pub fn new() -> Self {
        Self(50)
    }

    pub fn turn(&mut self, clicks: u32, left: bool) {
        if left {
            let clicks = clicks % 100;

            if clicks <= self.0 {
                self.0 -= clicks;
            } else {
                self.0 = 100 - (clicks - self.0);
            }
        } else {
            self.turn(100 - clicks % 100, true);
        }
    }
}

pub fn task1(input: &str) -> u32 {
    let mut dial = Dial::new();
    let mut zeros = 0;
    for line in input.lines() {
        let left = line.chars().nth(0).unwrap() == 'L';
        let clicks = &line[1..].parse::<u32>().unwrap();
        dial.turn(*clicks, left);
        if dial.0 == 0 {
            zeros += 1;
        }
    }
    zeros
}

pub fn task2(input: &str) -> u32 {
    let mut dial = Dial::new();
    let mut zeros = 0;

    for line in input.lines() {
        let left = line.chars().nth(0).unwrap() == 'L';
        let clicks = &line[1..].parse::<u32>().unwrap();

        let mut sum;
        if left {
            sum = clicks / 100;
            if clicks % 100 >= dial.0 && dial.0 != 0 {
                sum += 1;
            }
        } else {
            sum = (dial.0 + clicks) / 100;
        }
        zeros += sum;
        dial.turn(*clicks, left);
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        assert_eq!(3, task1(&input));
        assert_eq!(6, task2(&input));
    }
}
