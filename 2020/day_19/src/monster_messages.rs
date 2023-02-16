use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn digest_rules(unprocessed_rules: Vec<String>) -> HashMap<u32, String> {
    // let re = RegexSet::new(&[
    //     r"\d: \"a\"",
    //     r"", // piped
    //     r"", // ones with only digits
    // ]).unwrap();
    let matcher = Regex::new(r#"^\d+: "a|b"$"#).unwrap();
    let mut rules = HashMap::<u32, String>::new();
    for raw_rule in unprocessed_rules {
        let colon_index = raw_rule.find(":").unwrap();
        let rule_index = raw_rule[..colon_index].parse::<u32>().unwrap();
        if matcher.is_match(&raw_rule) {
            if raw_rule.contains("a") { rules.insert(rule_index, "a".to_string()); } 
            else { rules.insert(rule_index, "b".to_string()); }
        } else {
            let rule_content = raw_rule[colon_index+2..].to_string();
            rules.insert(rule_index, rule_content);
        }
    }
    rules
}

fn turn_rules_to_regex(rules: &mut HashMap<u32, String>) {
    let int_matcher = Regex::new(r"\d+").unwrap();
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
    use regex::Regex;
    use super::*;

    #[test]
    fn rules_and_messages_correctly_registered() {
        let (rules, messages) = get_input_from("test_data/test1.txt");
        assert_eq!(6, rules.len());
        assert_eq!(5, messages.len());
    }

    #[test]
    fn find_correct_matchers_for_dfferent_rules() {
        let matcher_1 = Regex::new(r#"^\d+: "a"$"#).unwrap();
        assert!(matcher_1.is_match("4: \"a\""));

        let matcher_2 = Regex::new(r#"^\d+: "b"$"#).unwrap();
        assert!(matcher_2.is_match("5: \"b\""));

        let matcher_3 = Regex::new(r"^\d+: \d+ \d+ \| (\d+) (\d+)$").unwrap();
        assert!(matcher_3.is_match("3: 4 5 | 5 4"));

        let matcher_4 = Regex::new(r"^\d+:( \d+)+$").unwrap();
        assert!(matcher_4.is_match("0: 4 1 5"));

        let string_regex_test = "^(ab|ba)$";
        let matcher_test = Regex::new(string_regex_test).unwrap();
        assert!(matcher_test.is_match("ab"));
        assert!(!matcher_test.is_match("abba"));
    }


    #[test]
    fn regex_for_quoted_rule_correctly_created() {
        let (rules, _) = get_input_from("test_data/test1.txt");
        let registered_rules = digest_rules(rules);
        assert_eq!("a", registered_rules[&4]);
    }

    #[test]
    fn update_all_rules_into_regex() {
        let (rules, _) = get_input_from("test_data/test1.txt");
        let mut rules = digest_rules(rules);
        turn_rules_to_regex(&mut rules);
        assert_eq!("(ab|ba)", rules[&3]);
    }
}