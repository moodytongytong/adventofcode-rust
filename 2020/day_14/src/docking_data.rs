use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

fn register_input_from(filepath: &str) -> Vec<String> {
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
}