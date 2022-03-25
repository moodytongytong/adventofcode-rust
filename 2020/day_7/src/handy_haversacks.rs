use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Rule;

impl Rule {
    fn new(&self, line: &str) -> Self {
        Rule
    }
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
    fn a_simple_rule_correctly_created() {
        let rule = Rule::new("bright white bags contain 1 shiny gold bag.");
    }

}