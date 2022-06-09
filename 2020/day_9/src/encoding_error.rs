use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn create_dataset_from(filepath: &str) -> Vec<usize> {
    let mut dataset = Vec::<usize>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(num) = line {
                dataset.push(num.parse().unwrap());
            }
        }
    }
    dataset
}

fn find_misbehaving_num_with_preamble(preamble_num: u8, dataset: &Vec<usize>) -> usize{
    let mut start_index = 0;
    let mut end_index = preamble_num as usize - 1;
    let mut possible_nums_to_add = HashSet::<usize>::new();
    for index in start_index..=end_index {
        possible_nums_to_add.insert(dataset[index]);
    }
    for index in preamble_num as usize..dataset.len() {
        // check if the number is the sum of two DISTINCT num from set
    }
    127
}

fn is_num_the_sum_of_two_from_set(target_sum: usize, possible_nums: &HashSet<usize>) -> bool {
    for num in possible_nums {
        if *num == (target_sum - num) {
            continue;
        }
        if possible_nums.contains(&(target_sum - num)) {
            return true;
        }
    }
    false
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
    fn dataset_correctly_created() {
        let dataset = create_dataset_from("test_data/test1.txt");
        assert_eq!(20, dataset.len());
        assert_eq!(35, dataset[0]);
        assert_eq!(576, dataset[19]);
    }

    #[test]
    fn test_first_irregular_num_correctly_found() {
        let dataset = create_dataset_from("test_data/test1.txt");
        assert_eq!(127, find_misbehaving_num_with_preamble(5, &dataset));
    }

    #[test]
    fn correctly_identify_if_a_number_is_the_sum_of_distinct_two_from_a_particular_set() {
        let mut possible_nums = HashSet::<usize>::new();
        for num in 1..=25 {
            possible_nums.insert(num);
        }
        assert!(is_num_the_sum_of_two_from_set(26, &possible_nums));
        assert!(is_num_the_sum_of_two_from_set(49, &possible_nums));
        assert_eq!(false, is_num_the_sum_of_two_from_set(100, &possible_nums));
        assert_eq!(false, is_num_the_sum_of_two_from_set(50, &possible_nums));
    }
}