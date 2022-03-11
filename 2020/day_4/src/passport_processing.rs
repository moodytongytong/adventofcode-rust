use std::fs::read_to_string;
use std::io;
use std::collections::{HashMap};
use regex::Regex;

#[derive(PartialEq, Eq)]
pub struct Record {
    fields: HashMap<String, String>
}



impl Record {
    // make map of functions available like a constant

    fn fields_are_valid(&self) -> bool {
        let mut validators = HashMap::<String, &dyn Fn(&str)->bool>::new();
        validators.insert("byr".to_string(), &is_byr_valid);
        validators.insert("iyr".to_string(), &is_iyr_valid);
        validators.insert("eyr".to_string(), &is_eyr_valid);
        validators.insert("hgt".to_string(), &is_hgt_valid);
        validators.insert("hcl".to_string(), &is_hcl_valid);
        validators.insert("ecl".to_string(), &is_ecl_valid);
        validators.insert("pid".to_string(), &is_pid_valid);

        for field in validators.keys() {
            let value = &self.fields[field];
            let func = validators[field];
            if func(value) == false {
                return false
            }
        }
        true
    }
}

fn new_record(info: &str) -> Record{
    let mut fields: HashMap<String, String> = HashMap::new();
    let attributes = info.split(&[' ', '\n'][..]);
    for attribute in attributes {
        let key_and_value: Vec<&str> = attribute.split(":").collect();
        fields.insert(key_and_value[0].to_string(), key_and_value[1].to_string());
    }
    Record {fields}
}


// COULD BE REFACTORED HERE
fn is_passport(record: &Record) -> bool {
    let passport_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in passport_fields {
        if record.fields.get(field) == None {
            return false;
        }
    }
    true
}

pub fn create_records_from_path(filepath: &str) -> Result<Vec<Record>, io::Error>{
    let raw_data = read_to_string(filepath)?;
    let data: Vec<&str> = raw_data[..].split("\n\n").collect();
    let mut records: Vec<Record> = Vec::new();
    for info in data {
        records.push(new_record(info));
    }
    Ok(records)
}

pub fn find_number_of_possible_passports(records: &Vec<Record>) -> usize {
    records.iter().filter(|&record| is_passport(record)).count()
}

pub fn find_number_of_valid_passports(records: &Vec<Record>) -> usize {
    records.iter().filter(|&record| is_passport(record)).filter(|&record| record.fields_are_valid()).count()
}

fn is_byr_valid(value: &str) -> bool {
    let date: usize = value.parse().unwrap();
    return date >= 1920 && date <= 2002;
}

fn is_iyr_valid(value: &str) -> bool {
    let date: usize = value.parse().unwrap();
    return date >= 2010 && date <= 2020;
}

fn is_eyr_valid(value: &str) -> bool {
    let date: usize = value.parse().unwrap();
    return date <= 2030 && date >= 2020;
}

fn is_hgt_valid(value: &str) -> bool {
    let num = &value[..value.len()-2];
    if value.contains("cm") {
        let num: usize = num.parse().unwrap();
        return num <= 193 && num >= 150;
    } else if value.contains("in") {
        let num: usize = num.parse().unwrap();
        return num <= 76 && num >= 59;
    } else {
        false
    }
}

fn is_hcl_valid(value: &str) -> bool {
    let hcl_format = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    hcl_format.is_match(value)
}

fn is_ecl_valid(value: &str) -> bool {
    let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    colors.contains(&value)
}

fn is_pid_valid(value: &str) -> bool {
    let pid_format = Regex::new(r"^[0-9]{9}$").unwrap();
    pid_format.is_match(value)
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn a_record_correctly_instantiated() {
        let info = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let record = new_record(info);
        assert_eq!(record.fields["ecl"], "gry");
        assert_eq!(record.fields["pid"], "860033327");
        assert_eq!(record.fields["eyr"], "2020");
        assert_eq!(record.fields["hcl"], "#fffffd");
        assert_eq!(record.fields["byr"], "1937");
        assert_eq!(record.fields["iyr"], "2017");
        assert_eq!(record.fields["cid"], "147");
        assert_eq!(record.fields["hgt"], "183cm");
    }

    #[test]
    fn full_passport_correctly_detected() {
        let info = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let record = new_record(info);
        assert!(is_passport(&record));
    }

    #[test]
    fn invalid_passport_correctly_identified() {
        let info = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929";
        let record = new_record(info);
        assert!(!is_passport(&record));
    }

    #[test]
    fn can_create_records_from_path() -> Result<(), io::Error> {
        let records = create_records_from_path("test_data/test1.txt")?;
        assert_eq!(4, records.len());
        Ok(())
    }

    #[test]
    fn number_of_valid_passports_correctly_found_for_part_1() -> Result<(), io::Error> {
        let records = create_records_from_path("test_data/test1.txt")?;
        assert_eq!(2, find_number_of_possible_passports(&records));
        Ok(())
    }

    #[test]
    fn check_byr_validity() {
        let valid_value = "1937";
        assert!(is_byr_valid(valid_value));
        let invalid_value = "2003";
        assert!(!is_byr_valid(invalid_value));
    }

    #[test]
    fn check_valid_iyr_is_valid() {
        let valid_value = "2015";
        assert!(is_iyr_valid(valid_value));
        let invalid_value = "2021";
        assert!(!is_iyr_valid(invalid_value));
    }

    #[test]
    fn check_eyr_validity() {
        let valid_value = "2020";
        assert!(is_eyr_valid(valid_value));
        let invalid_value = "2019";
        assert!(!is_eyr_valid(invalid_value));
    }

    #[test]
    fn check_hgt_validity() {
        let valid_value_cm = "190cm";
        assert!(is_hgt_valid(valid_value_cm));
        let valid_value_in = "60in";
        assert!(is_hgt_valid(valid_value_in));
        let invalid_value = "88";
        assert!(!is_hgt_valid(invalid_value));
    }

    #[test]
    fn check_hcl_validity() {
        let valid_value = "#123abc";
        assert!(is_hcl_valid(valid_value));
        let invalid_value = "#123abz";
        assert!(!is_hcl_valid(invalid_value));
    }

    #[test]
    fn check_ecl_validity() {
        let valid_value = "brn";
        assert!(is_ecl_valid(valid_value));
        let invalid_value = "wat";
        assert!(!is_ecl_valid(invalid_value));
    }

    #[test]
    fn check_pid_validity() {
        let valid_value = "000000001";
        assert!(is_pid_valid(valid_value));
        let invalid_value = "0123456789";
        assert!(!is_pid_valid(invalid_value));
    }

    #[test]
    fn check_all_mandatory_fields_in_record_valid() {
        let info = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let record = new_record(info);
        assert!(record.fields_are_valid());
    }

    #[test]
    fn record_with_all_necessary_fields_but_failing_values_fail() {
        let info = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719blahblah";
        let record = new_record(info);
        assert!(!record.fields_are_valid());
    }
}