fn main() {
    let input = include_str!("../input.txt");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

pub fn task1(input: &str) -> u64 {
    let mut problems: Vec<Vec<u64>> = vec![];
    let mut ops_plus = vec![];

    for line in input.lines() {
        let parts = line.split(" ").filter(|&x| x != "").collect::<Vec<_>>();
        for i in 0..parts.len() {
            let part: &&str = parts.get(i).unwrap();
            if let Ok(x) = part.parse::<u64>() {
                if let Some(v) = problems.get_mut(i) {
                    v.push(x);
                } else {
                    problems.push(vec![x]);
                }
            } else {
                ops_plus.push(part == &"+");
            }
        }
    }

    let mut sum = 0;
    for i in 0..problems.len() {
        if ops_plus[i] {
            sum += problems[i].iter().sum::<u64>()
        } else {
            sum += problems[i].iter().product::<u64>()
        }
    }
    sum
}

pub fn task2(input: &str) -> u64 {
    // Here lies my first attempt. Honestly I believe it could work but Im giving up, I think I have an easier way.

    // lets transpose all the text except the last line, and call task1 again
    // let n_lines = input.lines().count();

    // let mut text = vec![];
    // let mut line_size = 0;

    // let mut lines = input.lines();
    // for _ in 0..n_lines - 1 {
    //     let line = lines.next().unwrap();
    //     let chars = line.chars().map(|x| x as u8).collect::<Vec<u8>>();
    //     text.push(chars);
    //     if line.len() > line_size {
    //         line_size = line.len();
    //     }
    // }

    // // before transposing, pad the end of each vec with spaces
    // // for the transpose to work well.
    // for v in &mut text {
    //     while v.len() < line_size {
    //         v.push(' ' as u8);
    //     }
    // }

    // let text = text.iter().flatten().map(|x| *x).collect::<Vec<u8>>();

    // // now we transpose, and turn this back to a string
    // let mut transposed = vec![0; text.len()];
    // transpose::transpose(&text, &mut transposed, n_lines - 1, line_size);

    // let transposed = transposed.iter().map(|&v| v as char).collect::<String>();
    // let mut new_input = String::new();
    // for i in 0..line_size {
    //     new_input.push_str(&transposed[i * (n_lines - 1)..(i + 1) * (n_lines - 1)]);
    //     new_input.push('\n');
    // }
    // new_input.push_str(lines.last().unwrap());
    // println!("{}", new_input.clone());
    // task1(&new_input)

    // new idea: read by columns like a good boy. seems order does not matter

    let lines = input.lines().collect::<Vec<&str>>();
    let line_size = lines.iter().map(|l| l.len()).max().unwrap();

    let mut sum = 0;

    let mut col_idx = 0;
    while col_idx < line_size {
        let op_plus = &lines.last().unwrap()[col_idx..col_idx + 1] == "+";

        let mut operands = vec![];

        let mut stop = false;
        while !stop {
            // read number vertically
            let values = (&lines[0..lines.len() - 1])
                .iter()
                // if no char, insert a space instead (filtered out later)
                .map(|l| l.chars().nth(col_idx).unwrap_or(' '))
                .collect::<Vec<char>>();

            // stop condition: if all values are spaces, then we move to the next op
            if values.iter().all(|&v| v == ' ') {
                stop = true;
                continue;
            }

            // add it to the operands
            let number = values
                .iter()
                .filter(|&c| *c != ' ')
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            operands.push(number);

            col_idx += 1;
        }
        // computer sub sum and add it to the total
        let sub = if op_plus {
            operands.iter().sum::<u64>()
        } else {
            operands.iter().product()
        };
        sum += sub;

        // move to next operation
        col_idx += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let input = include_str!("../example.txt");
        assert_eq!(task1(&input), 4277556);
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example.txt");
        assert_eq!(task2(&input), 3263827);
    }
}
