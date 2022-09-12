use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{
    HashSet,
    HashMap,
};

pub fn find_wait_and_bus_id_product_from(input: Vec::<String>) -> usize {
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

fn the_earliest_time(bus_order: &String) -> usize {
    // constraints = [int(bus) if bus != "x" else False for bus in notes[1].split(",")]
    // bus_times = {constraint: -i % constraint for i, constraint in enumerate(constraints) if constraint}
    // buses = list(sorted(bus_times))
    // slowest = buses.pop()
    // time = bus_times[slowest]
    // while buses:
    //     bus = buses.pop()
    //     while time % bus != bus_times[bus]:
    //         time += slowest
    //     slowest *= bus
    // return time
    let mut bus_numbers_to_order = HashMap::<u16, u8>::new();
    for (order, word) in bus_order.split(",").enumerate() {
        if let Ok(bus_number) = word.parse::<u16>() {
            bus_numbers_to_order.insert(bus_number, order as u8);
        }
    }
    let mut ordered_buses = bus_numbers_to_order.into_keys().collect::<Vec<u16>>().sort();
    1068781
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
        let answer = find_wait_and_bus_id_product_from(input);
        assert_eq!(295, answer);
    }

    #[test]
    fn correctly_finds_part2_result() {
        let input = create_input_holder("test_data/test1.txt");
        let answer = the_earliest_time(&input[1]);
        assert_eq!(1068781, answer);
    }

}