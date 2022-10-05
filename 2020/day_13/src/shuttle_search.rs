use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{
    HashSet,
    HashMap,
};
use modinverse::modinverse;

pub fn find_wait_and_bus_id_product_from(input: &Vec::<String>) -> usize {
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

pub fn create_input_holder(filepath: &str) -> Vec<String> {
    let mut input = Vec::<String>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(content) = line {
                input.push(content);
            }
        }
    }
    input
}

// using the Chinese remainder theorem
pub fn the_earliest_time(bus_order: &String) -> usize {
    let mut bus_numbers_to_order = HashMap::<u16, u8>::new();
    for (order, word) in bus_order.split(",").enumerate() {
        if let Ok(bus_number) = word.parse::<u16>() {
            bus_numbers_to_order.insert(bus_number, order as u8);
        }
    }

    let ordered_buses: Vec<&u16> = bus_numbers_to_order.keys().collect();
    let id_product: isize  = ordered_buses.iter().fold(1, |product, &&id| id as isize * product);
    let mut time = 0;
    for (id, order) in bus_numbers_to_order {
        let remainder = order as usize;
        let n_i = id_product / id as isize;
        let x_i = modinverse(n_i as isize, id as isize);
        let product_i = remainder * n_i as usize * x_i.unwrap() as usize;
        time += product_i;
    }
    id_product as usize - time.rem_euclid(id_product as usize)
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
        let input = create_input_holder("test_data/test1.txt");
        let answer = find_wait_and_bus_id_product_from(&input);
        assert_eq!(295, answer);
    }

    #[test]
    fn correctly_finds_part2_result() {
        let input = create_input_holder("test_data/test1.txt");
        let answer = the_earliest_time(&input[1]);
        assert_eq!(1068781, answer);
    }

    #[test]
    fn correctly_finds_part2_result_2() {
        let answer = the_earliest_time(&"17,x,13,19".to_string());
        assert_eq!(3417, answer);
    }

    #[test]
    fn correctly_finds_part2_result_3() {
        let answer = the_earliest_time(&String::from("67,7,59,61"));
        assert_eq!(754018, answer);
    }

    #[test]
    fn correctly_finds_part2_result_4() {
        let answer = the_earliest_time(&String::from("67,x,7,59,61"));
        assert_eq!(779210, answer);
    }

    #[test]
    fn correctly_finds_part2_result_5() {
        let answer = the_earliest_time(&String::from("67,7,x,59,61"));
        assert_eq!(1261476, answer);
    }

    #[test]
    fn correctly_finds_part2_result_6() {
        let answer = the_earliest_time(&String::from("1789,37,47,1889"));
        assert_eq!(1202161486, answer);
    }

}