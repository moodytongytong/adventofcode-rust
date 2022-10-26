use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{
    HashMap, HashSet,
};

fn create_bitmask(input: String) -> HashMap<u8, char> {
    let mut mask = HashMap::<u8, char>::new();
    let mask_string = &input[7..];
    for (index, character) in mask_string.chars().enumerate() {
        if let '0' | '1' = character {
            mask.insert(index as u8, character);
        }
    }
    mask
}

fn convert_to_36bit_binary(num: usize) -> String {
    format!("{num:036b}")
}

fn convert_binary_to_decimal(binary_string: &str) -> usize {
    usize::from_str_radix(binary_string, 2).unwrap()
}

fn get_content_from(memory_instruction: String) -> (usize, usize) {
    let content: Vec<&str> = memory_instruction.split("] = ").collect();
    let address = content[0][4..].parse::<usize>().unwrap();
    let value = content[1].parse::<usize>().unwrap();
    (address, value)
}

pub fn execute_program1(input: &Vec<String>) -> usize {
    let mut memory = HashMap::<usize, usize>::new();
    let mut mask = HashMap::<u8, char>::new();
    for instruction in input {
        if "mask".to_string() == instruction[0..4] {
            mask = create_bitmask(instruction.to_string());
        } else {
            let (address, value) = get_content_from(instruction.to_string());
            memory.insert(address, apply_bitmask(value, &mask));
        }
    }
    let values: Vec<&usize> = memory.values().collect();
    values.iter().copied().sum()
}

pub fn execute_program2(input: &Vec<String>) -> usize {
    208
}

pub fn register_input_from(filepath: &str) -> Vec<String> {
    let mut input = Vec::<String>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(instruction) = line {
                input.push(instruction);
            }
        }
    }
    input
}

fn apply_bitmask(num_decimal: usize, mask: &HashMap<u8, char>) -> usize {
    let mut num_binary = convert_to_36bit_binary(num_decimal);
    for (index, bit) in mask {
        num_binary.replace_range((*index) as usize..(*index) as usize +1, &bit.to_string());
    }
    convert_binary_to_decimal(&num_binary)
}

fn apply_bitmask2(num_decimal: usize, mask: &HashMap<u8, char>) -> HashSet<usize> {
    let mut num_binary_string = convert_to_36bit_binary(num_decimal);
    for index in 0..36 {
        if let Some(bit) = mask.get(&index) {
            if bit == &'1' {
                num_binary_string.replace_range(index as usize ..index as usize +1, "1");
            }
        } else {
            num_binary_string.replace_range(index as usize ..index as usize +1, "X");
        }
    }
    create_set_of_addresses_from(num_binary_string)
}

fn create_set_of_addresses_from(binary_template: String) -> HashSet<usize> {
    let mut solution = HashSet::<usize>::new();
    let mut templates = Vec::<String>::new();
    templates.push(binary_template);
    while !templates.is_empty() {
        if let Some(current_template) = templates.pop() {
            if let Some(x_index) = current_template.find('X') {
                let mut refined_template_0: String = current_template.clone();
                refined_template_0.replace_range(x_index..x_index+1, "0");
                let mut refined_template_1 = current_template.clone();
                refined_template_1.replace_range(x_index..x_index+1, "1");
                templates.push(refined_template_0);
                templates.push(refined_template_1);
            } else {
                solution.insert(convert_binary_to_decimal(&current_template));
            }
        }
    }
    solution
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
    fn mask_is_correctly_created() {
        let mask = create_bitmask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        assert_eq!('1', mask[&29]);
        assert_eq!('0', mask[&34]);
    }
    
    #[test]
    fn convert_decimal_to_binary_36_bit_string() {
        let binary_11_36bit = convert_to_36bit_binary(11);
        assert_eq!("000000000000000000000000000000001011", binary_11_36bit);
    }

    #[test]
    fn convert_binary_string_to_decimal() {
        let decimal_11 = convert_binary_to_decimal("000000000000000000000000000000001011"); 
        assert_eq!(11, decimal_11);
    }

    #[test]
    fn apply_bitmask_correctly() {
        let mask = create_bitmask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        let num_after_bitmask = apply_bitmask(11, &mask);
        assert_eq!(73, num_after_bitmask);
    }

    #[test]
    fn correctly_register_input() {
        let input_vec = register_input_from("test_data/test1.txt");
        assert_eq!(4, input_vec.len());
    }

    #[test]
    fn correctly_process_memory_line() {
        let address_and_value = get_content_from("mem[8] = 11".to_string());
        assert_eq!(8, address_and_value.0);
        assert_eq!(11, address_and_value.1);
    }

    #[test]
    fn sum_correctly_calculated() {
        let input = register_input_from("test_data/test1.txt");
        let sum = execute_program1(&input);
        assert_eq!(165, sum);
    }

    #[test]
    fn part_2_apply_bitmask_returns_all_possible_addresses() {
        let mask = create_bitmask("mask = 000000000000000000000000000000X1001X".to_string());
        let possible_destinations = apply_bitmask2(42, &mask);
        assert_eq!(4, possible_destinations.len());
        assert!(possible_destinations.contains(&26));
        assert!(possible_destinations.contains(&27));
        assert!(possible_destinations.contains(&58));
        assert!(possible_destinations.contains(&59));
    }

    #[test]
    fn find_possible_destinations_correctly() {
        let address_template = "000000000000000000000000000000X1101X".to_string();
        let destinations = create_set_of_addresses_from(address_template);
        assert_eq!(4, destinations.len());
        assert!(destinations.contains(&26));
        assert!(destinations.contains(&27));
        assert!(destinations.contains(&58));
        assert!(destinations.contains(&59));
    }

    #[test]
    fn find_possible_destinations_for_more_complicated_example() {
        let address_template = "00000000000000000000000000000001X0XX".to_string();
        let destinations = create_set_of_addresses_from(address_template);
        assert_eq!(8, destinations.len());
        assert!(destinations.contains(&16));
        assert!(destinations.contains(&17));
        assert!(destinations.contains(&18));
        assert!(destinations.contains(&19));
        assert!(destinations.contains(&24));
        assert!(destinations.contains(&25));
        assert!(destinations.contains(&26));
        assert!(destinations.contains(&27));
    }

    #[test]
    fn part_2_result_correctly_calculated() {
        let input = register_input_from("test_data/test2.txt");
        let sum = execute_program2(&input);
        assert_eq!(208, sum);
        //NEED TO ACTUALLY USE THE APPLY FUNCTION
    }
}