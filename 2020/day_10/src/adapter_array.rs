use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn create_adapters(filepath: &str) -> Vec<usize> {
    let mut adapters = Vec::<usize>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(number) = line {
                adapters.push(number.parse().unwrap());
            } 
        }
    }
    adapters
}

pub fn find_differences_of_1_and_3_jolts_from(mut adapters: Vec<usize>) -> (usize, usize) {
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);
    let mut one_difference_count = 0;
    let mut three_difference_count = 0;
    for index in 1..adapters.len() {
        if let 1 = adapters[index] - adapters[index-1] {
            one_difference_count += 1;
        } else {
            three_difference_count += 1;
        }
    }
    (one_difference_count, three_difference_count)
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
    fn correctly_create_list_of_adapters() {
        let adapters = create_adapters("test_data/test1.txt");
        assert_eq!(11, adapters.len());
        assert_eq!(16, adapters[0]);
        assert_eq!(4, adapters[10]);
    }

    #[test]
    fn correctly_find_differences_of_1_and_3_jolts() {
        let adapters = create_adapters("test_data/test1.txt");
        let result = find_differences_of_1_and_3_jolts_from(adapters);
        assert_eq!((7, 5), result);
    }

    #[test]
    fn correctly_find_differences_of_1_and_3_jolts_on_more_complicated_example() {
        let adapters = create_adapters("test_data/test2.txt");
        let result = find_differences_of_1_and_3_jolts_from(adapters);
        assert_eq!((22, 10), result);
    }

}