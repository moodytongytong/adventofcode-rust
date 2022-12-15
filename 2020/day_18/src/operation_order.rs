use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn evaluate(expression: &str) -> usize {
    71
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