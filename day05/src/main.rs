fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

pub fn task1(input: &str) -> u64 {
    let mut ranges = vec![];
    let mut sum = 0;
    let mut inv = false;
    for line in input.lines() {
        if line == "" {
            inv = true;
            continue;
        }

        if !inv {
            let parts = line.split("-").collect::<Vec<_>>();
            let inf = parts.first().unwrap().parse::<u64>().unwrap();
            let sup = parts.last().unwrap().parse::<u64>().unwrap();
            ranges.push(inf..sup + 1);
        } else {
            let id = line.parse::<u64>().unwrap();
            for range in &ranges {
                if range.contains(&id) {
                    sum += 1;
                    break;
                }
            }
        }
    }
    sum
}

pub fn task2(input: &str) -> u64 {
    let mut ranges = vec![];
    for line in input.lines() {
        if line == "" {
            break;
        }

        let parts = line.split("-").collect::<Vec<_>>();
        let inf = parts.first().unwrap().parse::<u64>().unwrap();
        let sup = parts.last().unwrap().parse::<u64>().unwrap();
        ranges.push(inf..sup + 1);
    }

    // first sort for ez procecessing
    ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));

    let mut fact = vec![];
    for i in 0..ranges.len() {
        let range = ranges.get(i).unwrap();

        // init
        if fact.len() == 0 {
            fact.push(range.clone());
            continue;
        }

        let last = fact.last().unwrap();

        // if start after the last, we can safely add
        if range.start >= last.end {
            fact.push(range.clone());
            continue;
        } else {
            // else, check the end. if bigger than the last end, add last.end..range.end
            if range.end > last.end {
                fact.push(last.end..range.end);
                continue;
            }
        }
        // the rest is already in
    }
    fact.iter().map(|r| r.end - r.start).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let input = include_str!("../example.txt");
        assert_eq!(task1(&input), 3);
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example.txt");
        assert_eq!(task2(&input), 14);
    }
}
