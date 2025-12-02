fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

pub struct Range(u64, u64);

impl Range {
    pub fn new(x: &str) -> Self {
        let p: Vec<&str> = x.split("-").collect();
        Self(
            p.first().unwrap().parse::<u64>().unwrap(),
            p.last().unwrap().parse::<u64>().unwrap(),
        )
    }

    pub fn invalids(&self) -> Vec<u64> {
        let mut v = vec![];
        for i in self.0..self.1 + 1 {
            if is_invalid(&i.to_string()) {
                v.push(i);
            }
        }
        v
    }

    pub fn invalids2(&self) -> Vec<u64> {
        let mut v = vec![];
        for i in self.0..self.1 + 1 {
            if is_invalid2(&i.to_string()) {
                v.push(i);
            }
        }
        v
    }
}

pub fn is_invalid(x: &str) -> bool {
    if x.len() % 2 != 0 {
        return false;
    }

    &x[..x.len() / 2] == &x[(x.len() / 2)..]
}

/// first we compute all the multiples of x.len()
/// then we split the number into parts and compare the first with the rest of them.
pub fn is_invalid2(x: &str) -> bool {
    if x.len() == 1 {
        return false;
    }

    let multiples = multiples(x.len());
    for m in multiples {
        // first we build the parts for the given multiple
        let mut parts = vec![];
        let mut i = x.len() / m;
        let mut rest = x;
        while i > 0 {
            let (part, r) = rest.split_at(m);
            parts.push(part);
            rest = r;
            i -= 1;
        }
        // then we compare the first part will all the other
        let first = parts.first().unwrap();
        if (&parts[1..]).iter().all(|x| x == first) {
            return true;
        }
    }
    false
}

/// for a 9 long number, we have [1,3]
/// for a 8 long number, we have [1, 2, 4]
pub fn multiples(x: usize) -> Vec<usize> {
    let mut m = vec![1];
    for i in 2..(x / 2 + 1) {
        if x % i == 0 {
            m.push(i);
        }
    }
    m
}

pub fn task1(input: &str) -> u64 {
    let mut ranges = vec![];
    for range in input.split(",") {
        ranges.push(Range::new(range));
    }
    ranges
        .iter()
        .map(|r| r.invalids().iter().map(|x| *x).sum::<u64>())
        .sum()
}

pub fn task2(input: &str) -> u64 {
    let mut ranges = vec![];
    for range in input.split(",") {
        ranges.push(Range::new(range));
    }
    ranges
        .iter()
        .map(|r| r.invalids2().iter().map(|x| *x).sum::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid() {
        assert!(is_invalid("11"));
        assert!(is_invalid("6464"));
        assert!(is_invalid("123123"));
        assert!(is_invalid("1188511885"));
        assert!(!is_invalid("1234123"));
        assert!(!is_invalid("232323"));
    }

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        assert_eq!(task1(&input), 1227775554);
    }

    #[test]
    fn test_multiples() {
        assert_eq!(multiples(6), vec![1, 2, 3]);
        assert_eq!(multiples(8), vec![1, 2, 4]);
        assert_eq!(multiples(9), vec![1, 3]);
    }

    #[test]
    fn test_invalid2() {
        assert!(is_invalid2("11"));
        assert!(is_invalid2("6464"));
        assert!(is_invalid2("123123"));
        assert!(is_invalid2("1188511885"));
        assert!(!is_invalid2("1234123"));
        assert!(is_invalid2("232323"));
        assert!(is_invalid2("111"));
        assert!(!is_invalid2("1"));
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../example.txt");
        assert_eq!(task2(&input), 4174379265);
    }
}
