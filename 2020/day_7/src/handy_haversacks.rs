use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap, HashMap};
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Content {
    content: BTreeMap<String, u8>,
}

impl Content {
    fn new(tokens: Vec<&str>) -> Self {
        let mut content = BTreeMap::<String, u8>::new();
        for token in tokens {
            if let "no other" | "" = token {
                break;
            }
            content.insert(token[2..].to_string(), token[0..1].parse::<u8>().unwrap());
        }
        Self {
            content,
        }
    }
}

fn tokenize(rule: &str) -> Vec<&str> {
    let separator = Regex::new(r" bags contain | bag[s., ]*").expect("Invalid regex");
    return separator.split(rule.trim_end()).collect();
}

fn get_rules(filepath: &str) -> HashMap<String, Content> {
    let mut rules = HashMap::<String, Content>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(rule_info) = line {
                let tokens = tokenize(&rule_info);
                rules.insert(tokens[0].to_string(), Content::new(tokens[1..].to_vec()));
            }
        }
    }
    rules
}

// #[derive(Debug, Hash, PartialEq, Eq)]
// enum Color {
//     White(String),
//     Gold(String),
// }

// fn get_components(phrase: &str) -> (&str, &str) {
//     let words: Vec<&str> = phrase.split(" ").collect();
//     (words[0], words[1])
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_tokenizer_works_on_simple_rules() {
        let tokens = tokenize("bright white bags contain 1 shiny gold bag.");
        assert_eq!("bright white", tokens[0]);
        assert_eq!("1 shiny gold", tokens[1]);
    }

    #[test]
    fn test_tokenizer_works_on_complex_rules() {
        let tokens = tokenize("dark olive bags contain 3 faded blue bags, 4 dotted black bags.");
        assert_eq!("dark olive", tokens[0]);
        assert_eq!("3 faded blue", tokens[1]);
        assert_eq!("4 dotted black", tokens[2]);
    }

    #[test]
    fn test_tokenizer_works_on_empty_bag_rule() {
        let tokens = tokenize("faded blue bags contain no other bags.");
        assert_eq!("faded blue", tokens[0]);
    }

    #[test]
    fn can_correctly_create_set_of_all_rules() {
        let rules = get_rules("test_data/test2.txt");
        let tokens_1 = tokenize("bright white bags contain 1 shiny gold bag.");
        let expected_content_1 = Content::new(tokens_1[1..].to_vec());
        let tokens_2 = tokenize("dark olive bags contain 3 faded blue bags, 4 dotted black bags.");
        let expected_content_2 = Content::new(tokens_2[1..].to_vec());
        let tokens_3 = tokenize("faded blue bags contain no other bags.");
        let expected_content_3 = Content::new(tokens_3[1..].to_vec());
        assert_eq!(3, rules.len());
        assert_eq!(expected_content_1, rules["bright white"]);
        assert_eq!(expected_content_2, rules["dark olive"]);
        assert_eq!(expected_content_3, rules["faded blue"]);
    }
}