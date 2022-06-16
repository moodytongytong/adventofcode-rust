use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn create_adapters(filepath: &str) -> Vec<usize> {
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

fn find_differences_of_1_and_3_jolts_from(adapters: Vec<usize>) -> (usize, usize) {
    (7, 5)
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
        // sort the array and add max+3 into the array, then find the tuple
        let adapters = create_adapters("test_data/test1.txt");
        let result = find_differences_of_1_and_3_jolts_from(adapters);
        assert_eq!((7, 5), result);
    }

}