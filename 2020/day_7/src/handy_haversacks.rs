use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap, HashSet};
// use std::str::FromStr;
use regex::Regex;
// use std::fmt;

#[derive(Hash, PartialEq, Eq)]
struct Rule {
    bag_color: String,
    content: BTreeMap<String, u8>,
}


fn tokenize(rule: &str) -> Vec<&str> {
    let separator = Regex::new(r" bags contain | bag[s., ]*").expect("Invalid regex");
    return separator.split(rule.trim_end()).collect();
}

impl Rule {
    fn new(line: &str) -> Self {
        let mut content = BTreeMap::<String, u8>::new();
        let tokens: Vec<&str> = tokenize(line);
        for n in 1..tokens.len()-1 {
            if let "no other" = tokens[n] {
                break;
            }
            content.insert(tokens[n][2..].to_string(), tokens[n][0..1].parse::<u8>().unwrap());
        }

        Rule {
            bag_color: tokens[0].to_string(),
            content,
        }
    }
}

fn get_rules(filepath: &str) -> Vec<Rule> {
    let mut rules = Vec::<Rule>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(rule_info) = line {
                rules.push(Rule::new(&rule_info));
            }
        }
    }
    rules
}

fn find_num_of_bag_colors_which_could_contain(target_bag_color: &str, rules: Vec<Rule>) -> usize {
    let mut possible_colors = HashSet::<&str>::new();
    let total = 0;
    // loop {
    //     for rule in &rules {
    //         if rule.content.contains_key(target_bag_color) {
    //             possible_colors.insert(&rule.bag_color);
    //             total += 1;
    //         }
    //     }
    // }
    total
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Color {
    White(String),
    Gold(String),
}

fn get_components(phrase: &str) -> (&str, &str) {
    let words: Vec<&str> = phrase.split(" ").collect();
    (words[0], words[1])
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
    fn test_a_simple_rule_correctly_created() {
        let rule = Rule::new("bright white bags contain 1 shiny gold bag.");
        assert_eq!("bright white", rule.bag_color);
        assert_eq!(1, rule.content["shiny gold"]);
    }

    #[test]
    fn test_a_complex_rule_correctly_created() {
        let rule = Rule::new("dark olive bags contain 3 faded blue bags, 4 dotted black bags.");
        assert_eq!("dark olive", rule.bag_color);
        assert_eq!(3, rule.content["faded blue"]);
        assert_eq!(4, rule.content["dotted black"]);
    }

    #[test]
    fn test_a_rule_for_empty_bag_correctly_created() {
        let rule = Rule::new("faded blue bags contain no other bags.");
        assert_eq!("faded blue", rule.bag_color);
        assert_eq!(0, rule.content.len());
    }

    #[test]
    fn can_correctly_create_set_of_all_rules() {
        let rules = get_rules("test_data/test1.txt");
        assert_eq!(9, rules.len());
    }

    // Need to implement the counting logic
    
    // #[test]
    // fn can_correctly_number_of_bag_colors_which_could_contain_a_particular_colored_bags() {
    //     let rules = get_rules("test_data/test1.txt");
    //     assert_eq!(4, find_num_of_bag_colors_which_could_contain("shiny gold", rules));  // How best to name this function?
    // }
}