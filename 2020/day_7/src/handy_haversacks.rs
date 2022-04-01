use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;
// use std::fmt;

struct Rule {
    bag_color: Color,
    content: HashMap<Color, u8>,
}

impl Rule {
    fn new(line: &str) -> Self {
        let mut content = HashMap::<Color, u8>::new();
        let words: Vec<&str> = line.split(" ").collect();
        let mut words_iter = words.iter();

        // for (index, word) in words.iter().enumerate() {
        //     let bag_num = word.parse::<u8>();
        //     if let Ok(num) = bag_num {
        //         content.insert(Color::from_str(concat!("shiny", "gold")).unwrap(), 1);
        //     }
        // }
        
        content.insert(Color::Gold("shiny".to_string()), 1);
        Rule {
            bag_color: Color::White(words[0].to_string()),
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

impl FromStr for Color {
    type Err = String;
    fn from_str(input: &str) -> Result<Color, String> {
        let components = get_components(input);
        match components.1 {
            "white" => Ok(Color::White(components.0.to_string())),
            "gold" => Ok(Color::Gold(components.0.to_string())),
            _ => Err("Unrecognized color".to_string()),
        }
    }
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
    fn a_simple_rule_correctly_created() {
        let rule = Rule::new("bright white bags contain 1 shiny gold bag.");
        assert_eq!(Color::White("bright".to_string()), rule.bag_color);
        assert_eq!(1, rule.content[&Color::Gold("shiny".to_string())]);
    }

    // the next thing to do is to succesfully parse a simple line of rule

}