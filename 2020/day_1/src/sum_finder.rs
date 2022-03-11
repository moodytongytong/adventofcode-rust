use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub struct Finder {
    numbers: HashSet<u32>,
}

impl Finder {
    pub fn new(path: &str) -> Self {
        let mut numbers: HashSet<u32> = HashSet::new();
        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(num) = line {
                    numbers.insert(num.parse().unwrap());
                }
            }
        }
        Finder {
            numbers,
        }
    }

    pub fn find_product_of_two_targets_sum_to(&self, sum: u32) -> u32 {
        for number in self.numbers.iter() {
            if (sum as i16 - *number as i16) < 0 {
                continue;
            }
            if self.numbers.contains(&(sum - number)) {
                return number * (sum - number);
            }
        }
        0
        // look into ternary operator in Rust
    }

    pub fn find_product_of_three_targets_sum_to(&self, sum: u32) -> u32 {
        for number in self.numbers.iter() {
            let two_sum_product = self.find_product_of_two_targets_sum_to(sum - number);
            if two_sum_product != 0 {
                return number * two_sum_product;
            }
        }
        0
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
    fn finder_could_correctly_find_two_targets() {
        let finder = Finder::new("./test_data/test1.txt");
        assert_eq!(finder.find_product_of_two_targets_sum_to(2020), 514579);
    }

    #[test]
    fn finder_could_correctly_find_three_targets() {
        let finder = Finder::new("./test_data/test1.txt");
        assert_eq!(finder.find_product_of_three_targets_sum_to(2020), 241861950);
    }
}