use std::fs::read_to_string;
use std::collections::{
    HashSet, HashMap
};
use std::cell::RefCell;


pub struct Input {
    ranges: HashMap<String, [(usize, usize); 2]>,
    my_ticket: Vec<usize>,
    other_tickets: RefCell<HashSet<Vec<usize>>>,
    invalid_ticekts: RefCell<HashSet<Vec<usize>>>,
    field_position: RefCell<HashMap<usize, String>>,
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

    fn populate_other_tickets_from(&self, input: &str) {
        let mut content = input.split("\n").collect::<Vec<&str>>();
        content.remove(0);
        for line in content {
            let mut ticket = Vec::<usize>::new();
            for word in line.split(",") {
                if let Ok(num) = word.parse::<usize>() {
                    ticket.push(num);
                }
            }
            self.other_tickets.borrow_mut().insert(ticket);
        }
    }

    pub fn find_sum_of_invalid_values(&self) -> usize {
        let mut invalid_total = 0;
        for ticket in self.other_tickets.borrow() {
            'number_loop: for number in ticket {
                for (_category, range_set) in &self.ranges {
                    for range in range_set {
                        if Self::is_number_valid(*number, *range) {continue 'number_loop;}
                    }
                }
                invalid_total += number;
                self.invalid_ticekts.borrow_mut().insert(ticket.to_vec());
            }
        }
        invalid_total
    }

    fn is_number_valid(number: usize, range: (usize, usize)) -> bool  {
        return number >= range.0 && number <= range.1;
    }

    fn find_categories_for_number(&self, number: usize) -> HashSet<&str> {
        let mut possible_categories = HashSet::<&str>::new();
        'category_loop: for (category, range_set) in &self.ranges {
            for range in range_set {
                if Self::is_number_valid(number, *range) {
                    possible_categories.insert(category);
                    continue 'category_loop;
                }
            }
        }
        possible_categories
    }

    fn find_category_for_position(&self, position: usize) -> &str {
        let mut result = self.ranges.keys().map(|category| category.as_str()).collect::<HashSet<_>>();
        for known_category in self.field_position.borrow().values() {
            result.remove(known_category as &str);
        }
        for ticket in self.other_tickets.borrow() {
            let number = ticket[position];
            result = result.intersection(&self.find_categories_for_number(number)).copied().collect::<HashSet<_>>();
            println!("number is {} and current result after intersection is {:?}", number, result)
        }
        println!("result is {:?}", result);
        let result = result.iter().next().unwrap().clone();
        self.field_position.borrow_mut().insert(position, result.to_string());
        &self.field_position.borrow()[&position] as &str
    }

    fn remove_invalid_tickets(&self) {
        for invalid_ticket in self.invalid_ticekts.borrow().iter() {
            self.other_tickets.borrow_mut().remove(invalid_ticket as &Vec<usize>);
        }
    }

    fn populate_field_position_map(&mut self) {
        for position in 0..self.my_ticket.len() {
            self.find_category_for_position(position);
        }
    }
}

pub fn create_formatted_input_from(filepath: &str) -> Input {
    let mut input = Input {
        ranges : HashMap::<String, [(usize, usize); 2]>::new(),
        my_ticket : Vec::<usize>::new(),
        other_tickets : RefCell::new(HashSet::<Vec<usize>>::new()),
        invalid_ticekts : RefCell::new(HashSet::<Vec<usize>>::new()),
        field_position : RefCell::new(HashMap::<usize, String>::new()),
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
        assert_eq!(4, info.other_tickets.borrow().len());

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

    #[test]
    fn find_set_of_possibilities_for_a_number() {
        let info = create_formatted_input_from("test_data/test2.txt");
        let set_of_cats = info.find_categories_for_number(3);
        let mut expected_set = HashSet::<&str>::new();
        expected_set.insert("row");
        expected_set.insert("seat");
        assert_eq!(expected_set, set_of_cats);
    }

    #[test]
    fn find_category_for_position_correctly() {
        let info = create_formatted_input_from("test_data/test2.txt");
        assert_eq!("row", info.find_category_for_position(0));
        assert_eq!("class", info.find_category_for_position(1));
        assert_eq!("seat", info.find_category_for_position(2));
    }

    #[test]
    fn ensure_invalid_tickets_are_removed() {
        let info = create_formatted_input_from("test_data/test1.txt");
        info.find_sum_of_invalid_values();
        info.remove_invalid_tickets();
        assert_eq!(1, info.other_tickets.borrow().len());
    }

    #[test]
    fn ensure_position_map_correctly_registered() {
        let info = create_formatted_input_from("test_data/test2.txt");
        info.populate_field_position_map();
        let mut expected = HashMap::<usize, String>::new();
        expected.insert(0, "row".to_string());
        expected.insert(1, "class".to_string());
        expected.insert(2, "seat".to_string());
        assert_eq!(expected, info.field_position.take());

    }
}