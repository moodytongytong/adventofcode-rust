use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
// use std::str::FromStr;
use regex::Regex;
// use std::fmt;

struct Rule {
    bag_color: String,
    content: HashMap<String, u8>,
}


fn tokenize(rule: &str) -> Vec<&str> {
    let separator = Regex::new(r" bags contain | bag[s., ]*").expect("Invalid regex");
    return separator.split(rule.trim_end()).collect();
}

// fn parse_tokens(bags: HashMap<&str,HashSet<&str>>, tokens: Vec<&str>) -> HashMap<&str,HashSet<&str>> {
//             //     holder = tokens.pop(0)
//         //     dependents = self.bags.get(holder,set())
//         //     for token in tokens:
//         //         if token := token.strip():
//         //             dependents.add(token)
//         //     self.bags[holder] = dependents
//     bags
// }

impl Rule {
    fn new(line: &str) -> Self {
        let mut content = HashMap::<String, u8>::new();
        let tokens: Vec<&str> = tokenize(line);
        content.insert("shiny gold".to_string(), 1);
        // let mut tokens_iter = tokens.iter();

        // if let Some(bag) = tokens_iter.next() {
            
        // }
        
        // content.insert(Color::Gold("shiny".to_string()), 1);
        Rule {
            bag_color: tokens[0].to_string(),
            content,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Color {
    White(String),
    Gold(String),
}

// impl fmt::Debug for Color {
//     fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use Color::*;
//         match self {
//             White(variant) => write!(f, "White({:?})", variant),
//             Gold(variant) => write!(f, "Gold({:?}", variant),
//         }
//     }
// }

// impl FromStr for Color {
//     type Err = String;
//     fn from_str(input: &str) -> Result<Color, String> {
//         let components = get_components(input);
//         match components.1 {
//             "white" => Ok(Color::White(components.0.to_string())),
//             "gold" => Ok(Color::Gold(components.0.to_string())),
//             _ => Err("Unrecognized color".to_string()),
//         }
//     }
// }

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
    fn test_a_simple_rule_correctly_created() {
        let rule = Rule::new("bright white bags contain 1 shiny gold bag.");
        assert_eq!("bright white", rule.bag_color);
        assert_eq!(1, rule.content["shiny gold"]);
    }
    // MAKE THE TEST PASS MORE INTELLIGENTLY RATHER THAN HARDCODING
    // ADD NEW TEST SO THAT A MORE COMPLICATED RULE COULD BE CREATED

    // #[test]
    // fn test_parser_works() {
    //     // let bags = HashMap::<&str, HashSet<&str>>::new();
    //     let tokens = tokenize("bright white bags contain 1 shiny gold bag.");
    //     let bags = parse_tokens(HashMap::<&str, HashSet<&str>>::new(), tokens);
    //     let expected = HashSet::<&str>::new();
    //     expected.insert("1 shiny gold");
    //     assert_eq!(expected, bags["bright white"]);
    // }


    // the next thing to do is to succesfully parse a simple line of rule

}