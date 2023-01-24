use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::LinkedList;

fn evaluate(all_chars: &mut LinkedList<char>) -> usize {
    let mut result = all_chars.pop_front().unwrap().to_digit(10).unwrap() as usize;
    let mut first_op = ' ';
    if let Some(op) = all_chars.pop_front() {
        match op {
            '+' => result += evaluate(all_chars) as usize,
            '*' => result *= evaluate(all_chars) as usize,
            // ')' => {
            //     let mut sub_expression: LinkedList<char> = LinkedList::new();
            //     let mut next_char = all_chars.pop_front().unwrap();
            //     while next_char != '(' {
            //         sub_expression.push_front();
            //         next_char = all_chars.pop_front().unwrap();
            //     }
            // }
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



fn evaluate_old(expression: &str) -> usize {
    // let lh = evaluate(expression[:-2])
    // let op = expression[-2]
    // let rh = expression[-1]
    // return op(lh, rh)
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
            '(' => {

            }
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
        let mut data = tokenize("1 + 2 * 3 + 4 * 5 + 6");
        let result = evaluate(&mut data);
        assert_eq!(71, result);
    }

    #[test]
    fn expression_with_parentheses_correctly_evaluated() {
        let result = evaluate("4 * (5 + 6)");
        assert_eq!(44, result);
    }
}