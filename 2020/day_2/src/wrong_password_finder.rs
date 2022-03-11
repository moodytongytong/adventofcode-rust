use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub fn new_records(filepath: &str) -> HashSet<Record> {
    let mut records: HashSet<Record> = HashSet::new();
        if let Ok(lines) = read_lines(filepath) {
            for line in lines {
                if let Ok(info) = line {
                    records.insert(Record::new(&info));
                }
            }
        }
    records
}

pub fn find_number_of_valid_records(records: &HashSet<Record>, policy: &dyn Fn(&Record) -> bool ) -> usize {
    records.iter().filter(|record| policy(record)).count()
}

#[derive(PartialEq, Eq, Hash)]
pub struct Record {
    password: String,
    constraints: Constraints,
}

impl Record {
    fn new(info: &str) -> Self {
        let info: Vec<&str> = info.split(": ").collect();
        Record {
            password: info[1].to_string(),
            constraints: Constraints::new(info[0]),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Constraints {
    nums: (u8, u8),
    target_letter: char,
}

impl Constraints {
    fn new(info: &str) -> Self {
        let info: Vec<&str> = info.split_whitespace().collect();
        let limits: Vec<&str> = info[0].split("-").collect();
        Self { 
            nums: (limits[0].parse().unwrap(), limits[1].parse().unwrap()),
            target_letter: info[1].parse().unwrap(),
        }
    }
}

pub fn policy_pt1(record: &Record) -> bool {
    let count = record.password.matches(record.constraints.target_letter).count() as u8;
    return count >= record.constraints.nums.0 && count <= record.constraints.nums.1;
}

pub fn policy_pt2(record: &Record) -> bool {
    let first = record.password.chars().nth(record.constraints.nums.0 as usize - 1).unwrap();
    let second = record.password.chars().nth(record.constraints.nums.1 as usize - 1).unwrap();
    let target = record.constraints.target_letter;
    (first == target && second != target) || (first != target && second == target)
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
    fn test_password_correctly_instantiated() {
        let record = Record::new("1-3 a: abcde");
        assert_eq!(record.password, "abcde");
        assert_eq!(record.constraints, Constraints::new("1-3 a"));
    }

    #[test]
    fn test_password_constraints_correctly_instantiated() {
        let constraints = Constraints::new("1-3 a");
        assert_eq!(constraints.nums, (1, 3));
        assert_eq!(constraints.target_letter, 'a');
    }
    
    #[test]
    fn password_validity_correctly_evaluated_for_part_1() {
        let valid = Record::new("1-3 a: abcde");
        assert!(policy_pt1(&valid));
    }
    
    #[test]
    fn invalid_password_correctly_evaluated_for_part_1() {
        let invalid = Record::new("1-3 b: cdefg");
        assert!(!policy_pt1(&invalid));
    }

    #[test]
    fn can_find_number_of_valid_records_correctly_for_part_1() {
        let records = new_records("test_data/test1.txt");
        assert_eq!(find_number_of_valid_records(&records, &policy_pt1), 2);
    }

    #[test]
    fn password_validity_correctly_evaluated_for_part_2() {
        let valid = Record::new("1-3 a: abcde");
        assert!(policy_pt2(&valid));
    }

    #[test]
    fn invalid_password_correctly_evaluated_for_part_2() {
        let invalid = Record::new("1-3 b: cdefg");
        assert!(!policy_pt2(&invalid));
    }
}