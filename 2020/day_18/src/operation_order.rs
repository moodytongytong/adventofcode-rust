use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::LinkedList;

fn evaluate(all_chars: &mut LinkedList<char>) -> usize {
    let mut result = if let Some(expression) = all_chars.pop_front() {
        match expression {
            ')' => {
                let mut parentheses_count = 1;
                let mut tokenized_sub_expression: LinkedList<char> = LinkedList::new();
                let mut next_char = all_chars.pop_front().unwrap();
                while next_char != '(' || parentheses_count != 0 {
                    if next_char == ')' { parentheses_count += 1; }
                    tokenized_sub_expression.push_back(next_char);
                    next_char = all_chars.pop_front().unwrap();
                    if next_char == '(' { parentheses_count -= 1; }
                }
                evaluate(&mut tokenized_sub_expression)
            },
            _ => expression.to_digit(10).unwrap() as usize,
        }
    } else {
        0
    };
    if let Some(op) = all_chars.pop_front() {
        match op {
            '+' => result += evaluate(all_chars) as usize,
            '*' => result *= evaluate(all_chars) as usize,
            _ => println!("DO NOTHING :D"),
        }
    }
    result
}

fn tokenize(expression: &str) -> LinkedList<char> {
    let mut all_chars: LinkedList<char> = LinkedList::new();
    for c in expression.chars() {
        if c != ' ' { all_chars.push_front(c); }
    }
    all_chars
}

pub fn find_sum_from_homework(filepath: &str) -> usize {
    let mut results: Vec<usize> = vec![];
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(expression) = line {
                let mut expression = tokenize(&expression);
                results.push(evaluate(&mut expression));
            }
        }
    }
    results.iter().sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn expression_without_parentheses_correctly_evaluated() {
        let mut data = tokenize("1 + 2 * 3 + 4 * 5 + 6");
        let result = evaluate(&mut data);
        assert_eq!(71, result);
    }

    #[test]
    fn expression_with_parentheses_correctly_evaluated() {
        let mut data = tokenize("4 * (5 + 6)");
        let result = evaluate(&mut data);
        assert_eq!(44, result);
    }

    #[test]
    fn expression_with_nested_parentheses_correctly_evaluated() {
        let mut data = tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        let result = evaluate(&mut data);
        assert_eq!(12240, result);
    }

    #[test]
    fn very_complicated_expression_correctly_evaluated() {
        let mut data = tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        let result = evaluate(&mut data);
        assert_eq!(13632, result);
    }

    #[test]
    fn find_part1_sum_correctly() {
        assert_eq!(26335, find_sum_from_homework("test_data/test1.txt"));
    }
}