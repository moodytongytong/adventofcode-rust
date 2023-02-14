use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn digest_rules(unprocessed_rules: Vec<String>) -> HashMap<usize, String> {
    let re = RegexSet::new(&[
        r"\d: \"a\"",
        r"", // piped
        r"", // ones with only digits
    ]).unwrap();

    for rule in unprocessed_rules {

    }
}



fn get_input_from(filepath: &str) -> (Vec<String>, Vec<String>) {
    let mut input: Vec<String> = vec![];
    let mut break_point_index = 0;

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(content) = line {
                if content.is_empty() { break_point_index = input.len(); }
                input.push(content);
            }
        }
    }
    let rules = input[0..break_point_index].to_vec();
    let messages = input[break_point_index+1..].to_vec();
    (rules, messages)
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
    fn rules_and_messages_correctly_registered() {
        let (rules, messages) = get_input_from("test_data/test1.txt");
        assert_eq!(6, rules.len());
        assert_eq!(5, messages.len());
    }

    #[test]
    fn match_rules_correctly() {
        let matcher = match("4: \"a\"");
    }


    // #[test]
    // fn regex_for_quoted_rule_correctly_created() {
    //     let (rules, messages) = get_input_from("test_data/test1.txt");
    //     let processed_rules = digest_rules(rules);
    //     assert_eq!("a", processed_rules[&4]);
    // }
}