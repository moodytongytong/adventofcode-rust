use std::fs::read_to_string;
use std::collections::{
    HashSet, HashMap
};


pub struct Input {
    ranges: HashMap<String, [(usize, usize); 2]>,
    my_ticket: Vec<usize>,
    other_tickets: HashSet<Vec<usize>>,
}

impl Input {
    fn populate_my_ticket_from(&mut self, input: &str) {
        let content = input.split("\n").collect::<Vec<&str>>()[1];
        for word in content.split(",") {
            if let Ok(num) = word.parse::<usize>() {
                self.my_ticket.push(num);
            }
        }
    }

    fn populate_ranges_from(&mut self, input: &str) {
        let content = input.split("\n").collect::<Vec<&str>>();
        for line in content {
            let rule = line.split(": ").collect::<Vec<&str>>();
            let name = rule[0].to_string();
            let ranges_expr = rule[1].split(" or ").collect::<Vec<&str>>();
            let mut values: [(usize, usize); 2] = [(0, 0); 2];
            for (idx, range_expr) in ranges_expr.iter().enumerate() {
                let numbers_expr = range_expr.split("-").collect::<Vec<&str>>();
                values[idx] = (numbers_expr[0].parse::<usize>().unwrap(), numbers_expr[1].parse::<usize>().unwrap());
            }
            self.ranges.insert(name, values);
        }
    }

    fn populate_other_tickets_from(&mut self, input: &str) {
        let mut content = input.split("\n").collect::<Vec<&str>>();
        content.remove(0);
        for line in content {
            let mut ticket = Vec::<usize>::new();
            for word in line.split(",") {
                if let Ok(num) = word.parse::<usize>() {
                    ticket.push(num);
                }
            }
            self.other_tickets.insert(ticket);
        }
    }

    pub fn find_sum_of_invalid_values(&self) -> usize {
        let mut invalid_total = 0;
        for ticket in &self.other_tickets {
            'number_loop: for number in ticket {
                for (_category, range_set) in &self.ranges {
                    for range in range_set {
                        if *number >= range.0 && *number <= range.1 {continue 'number_loop;}
                    }
                }
                invalid_total += number;
            }
        }
        invalid_total
    }
}

pub fn create_formatted_input_from(filepath: &str) -> Input {
    let mut input = Input {
        ranges : HashMap::<String, [(usize, usize); 2]>::new(),
        my_ticket : Vec::<usize>::new(),
        other_tickets : HashSet::<Vec<usize>>::new(),
    };
    if let Ok(raw_input) = read_to_string(filepath) {
        let three_sections: Vec<&str> = raw_input.split("\n\n").collect();
        input.populate_ranges_from(three_sections[0]);
        input.populate_my_ticket_from(three_sections[1]);
        input.populate_other_tickets_from(three_sections[2]);
    }
    input
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn info_correctly_registered() {
        let info = create_formatted_input_from("test_data/test1.txt");
        assert_eq!(3, info.my_ticket.len());
        assert_eq!(3, info.ranges.len());
        assert_eq!(4, info.other_tickets.len());

        assert_eq!([(1, 3), (5, 7)], info.ranges["class"]);
        assert_eq!([(6, 11), (33, 44)], info.ranges["row"]);
        assert_eq!([(13, 40), (45, 50)], info.ranges["seat"]);

        assert!(info.my_ticket.contains(&7));
        assert!(info.my_ticket.contains(&1));
        assert!(info.my_ticket.contains(&14));
    }

    #[test]
    fn sum_invalid_values_correctly() {
        let info = create_formatted_input_from("test_data/test1.txt");
        assert_eq!(71, info.find_sum_of_invalid_values());
    }
}