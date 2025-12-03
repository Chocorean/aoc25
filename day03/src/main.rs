fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

/// we provide to option to ignore a specific value
/// first call (ignore == None) should ignore the last digit
/// second call should ignore all the figures before the returned index (included)
pub fn highest(bank: &str, ignore: Option<usize>) -> (u64, usize) {
    let str = if let Some(idx) = ignore {
        String::from(&bank[idx + 1..])
    } else {
        String::from(&bank[0..bank.len() - 1])
    };
    let (mut max, mut idx) = (0, 0);
    for i in 0..str.len() {
        let v = (&str[i..i + 1]).parse::<u64>().unwrap();
        if v > max {
            max = v;
            idx = i;
        }
    }
    (max, idx)
}

pub fn task1(input: &str) -> u64 {
    let mut sum = 0;
    for bank in input.lines() {
        let (high, high_pos) = highest(bank, None);
        let (low, _) = highest(bank, Some(high_pos));
        let sub = high * 10 + low;
        sum += sub;
    }
    sum
}

/// the difference being we dont consider all the next candidates,
/// only those who are left enough so there are enough figures to their right
///
/// left_ignored is the index of the first NOT IGNORED digit
fn highest12(bank: &str, left_ignored: usize, size: usize) -> (u64, usize) {
    let str: Vec<(usize, char)> = bank
        .chars()
        .enumerate()
        .into_iter()
        .filter(|(idx, _)| *idx >= left_ignored)
        .filter(|(idx, _)| *idx < bank.len() - (12 - 1 - size))
        .collect();

    let (mut max, mut idx) = (0, 0);
    for (i, c) in str {
        let v = c.to_string().parse::<u64>().unwrap();
        if v > max {
            max = v;
            idx = i;
        }
    }
    (max, idx)
}

pub fn task2(input: &str) -> u64 {
    let mut sum = 0;
    for bank in input.lines() {
        let mut sub = 0;
        let mut pos = 0;
        for i in 0..12 {
            let (high, high_pos) = highest12(bank, pos, i);
            sub += high * 10_u64.pow(12 - 1 - i as u32);
            pos = high_pos + 1;
        }
        sum += sub;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let input = include_str!("../example.txt");
        assert_eq!(task1(&input), 357);
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example.txt");
        assert_eq!(task2(&input), 3121910778619);
    }
}
