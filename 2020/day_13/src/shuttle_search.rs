use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub fn find_product_from(filepath: &str) -> usize {
    let mut input = Vec::<String>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(content) = line {
                input.push(content);
            }
        }
    }
    let earliest_departure_time: usize = input[0].parse().unwrap();
    let mut bus_numbers = HashSet::<u16>::new();
    for word in input[1].split(",") {
        if let Ok(bus_number) = word.parse::<u16>() {
            bus_numbers.insert(bus_number);
        }
    }
    let mut departure_time = earliest_departure_time;
    loop {
        for bus_number in &bus_numbers {
            if departure_time % *bus_number as usize == 0 {
                return *bus_number as usize * (departure_time - earliest_departure_time);
            }
        }
        departure_time += 1;
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
    fn correctly_find_the_product() {
        let answer = find_product_from("test_data/test1.txt");
        assert_eq!(295, answer);
    }

}