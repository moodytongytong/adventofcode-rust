use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::LinkedList;

fn evaluate(expression: &str) -> usize {
    let mut nums: LinkedList<u8> = LinkedList::new();
    let mut operators: LinkedList<char> = LinkedList::new();
    for c in expression.chars() {
        if let Some(num) = c.to_digit(10) { nums.push_back(num as u8);} 
        else { if c != ' ' { operators.push_back(c)}; }
    }
    let mut current_result = nums.pop_front().unwrap() as usize;
    for operator in operators {
        let next_num = nums.pop_front().unwrap();
        match operator {
            '+' => current_result += next_num as usize,
            '*' => current_result *= next_num as usize,
            '-' => current_result -= next_num as usize,
            '/' => current_result /= next_num as usize,
            _ => println!("DO NOTHING :D"),
        }
    }
    current_result
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
        let result = evaluate("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(71, result);
    }

    // #[test]
    // fn expression_with_parentheses_correctly_evaluated() {
    //     let result = evaluate("1 + (2 * 3) + (4 * (5 + 6))");
    //     assert_eq!(51, result);
    // }
}