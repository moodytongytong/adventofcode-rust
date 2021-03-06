use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BTreeMap, HashMap, HashSet};
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Content {
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

    fn eventually_contains(&self, target_color: &str, rules: &HashMap<String, Content>) -> bool {
        if self.content.contains_key(target_color) {
            return true;
        }
        if self.content.is_empty() {
            return false;
        }
        let mut result = false;
        for color in self.content.keys() {
            result = result || rules[color].eventually_contains(target_color, rules);
        }
        result
    }
}

fn tokenize(rule: &str) -> Vec<&str> {
    let separator = Regex::new(r" bags contain | bag[s., ]*").expect("Invalid regex");
    return separator.split(rule.trim_end()).collect();
}

pub fn get_rules(filepath: &str) -> HashMap<String, Content> {
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

pub fn find_num_of_colors_that_could_contain(target_color: &str, rules: &HashMap<String, Content>) -> usize {
    let mut possible_colors = HashSet::<String>::new();
    for (color, content) in rules.iter() {
        for key in content.content.keys() {
            if possible_colors.contains(key) {
                possible_colors.insert(color.clone());
                break;
            }
        }
        if content.eventually_contains(target_color, &rules) {
            possible_colors.insert(color.clone());
        }
    }
    possible_colors.len()
}

pub fn find_num_of_bags_contained_in(target_color: &str, rules: &HashMap<String, Content>) -> usize {
    if rules[target_color].content.is_empty() {
        return 0;
    } else {
        let mut total: usize = 0;
        for (color, number) in &rules[target_color].content {
            total += *number as usize * find_num_of_bags_contained_in(&color, rules) as usize + *number as usize;
        }
        return total;
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
        let rules = get_rules("test_data/test2_part1.txt");
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

    #[test]
    fn can_obtain_number_of_colors_containing_a_particular_color_for_an_example_with_one_level_of_nexting() {
        let rules = get_rules("test_data/test1_part1.txt");
        assert_eq!(4, find_num_of_colors_that_could_contain("shiny gold", &rules));
    }

    #[test]
    fn can_obtain_number_of_colors_containing_a_particular_color_for_an_example_with_two_levels_of_nexting() {
        let rules = get_rules("test_data/test3_part1.txt");
        assert_eq!(5, find_num_of_colors_that_could_contain("shiny gold", &rules));
    }

    #[test]
    fn can_obtain_the_number_of_bags_an_empty_color_contains() {
        let rules = get_rules("test_data/test5_part2.txt");
        assert_eq!(0, find_num_of_bags_contained_in("dotted black", &rules));
    }

    #[test]
    fn can_obtain_the_number_of_bags_contained_in_a_simple_color() {
        let rules = get_rules("test_data/test5_part2.txt");
        assert_eq!(7, find_num_of_bags_contained_in("dark olive", &rules));
    }

    #[test]
    fn can_obtain_the_number_of_bags_a_color_contains() {
        let rules = get_rules("test_data/test4_part2.txt");
        assert_eq!(126, find_num_of_bags_contained_in("shiny gold", &rules));
    }
}