fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

pub fn task1(input: &str) -> u64 {
    0
}

pub fn task2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let input = include_str!("../example.txt");
        assert_eq!(task1(&input), 0);
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example.txt");
        assert_eq!(task2(&input), 0);
    }
}
