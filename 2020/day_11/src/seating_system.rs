use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{
    HashMap,
    HashSet,
};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord (usize, usize);

impl Coord {
    fn get_neighbors(&self) -> HashSet<Coord> {
        let mut neighbors = HashSet::new();
        if self.0 != 0 && self.1 != 0 { 
            neighbors.insert(Coord(self.0 - 1, self.1 - 1));
        }
        if self.0 != 0 {
            neighbors.insert(Coord(self.0 - 1, self.1));
            neighbors.insert(Coord(self.0 - 1, self.1 + 1));
        }
        if self.1 != 0 {
            neighbors.insert(Coord(self.0, self.1 - 1));
            neighbors.insert(Coord(self.0 + 1, self.1 - 1));
        }
        neighbors.insert(Coord(self.0, self.1 + 1));
        neighbors.insert(Coord(self.0 + 1, self.1));
        neighbors.insert(Coord(self.0 + 1, self.1 + 1));
        neighbors
        // What's a cleaner way to write this without getting stack overflow
    }
}

struct WaitingArea {
    seat_map: HashMap<Coord, bool>
}

impl WaitingArea {
    fn new(filepath: &str) -> Self {
        let seat_map = create_map(filepath);
        Self { seat_map }
    }

    fn is_seat(&self, location: &Coord) -> bool {
        self.seat_map.contains_key(location)
    }

    fn is_occupied(&self, location: &Coord) -> bool {
        if let Some(occupied) = self.seat_map.get(location) {
            return *occupied;
        } else {
            return false;
        }
    }

    fn update(&mut self) {
        let mut new_map = HashMap::<Coord, bool>::new();
        for (location, occupied) in &self.seat_map {
            // determine its status or whether it changes and add the new status to the new map
            if *occupied {
                // use rules for occupied seats)
            }
            else {
                // use rule for empty setats
            }
            // refactor this to make it more functional
        }
        self.seat_map = new_map;
    }

    fn apply_rule_for_occupied_seat_on(&self, location: &Coord) -> bool {
        self.find_number_of_occupied_neighbors(location) < 4
    }

    fn apply_rule_for_empty_seat_on(&self, location: &Coord) -> bool {
       self.find_number_of_occupied_neighbors(location) == 0
    }

    fn find_number_of_occupied_neighbors(&self, location: &Coord) -> u8 {
        let neighbors = location.get_neighbors();
        let mut occupied_neighbors_count: u8 = 0;
        for neighbor in &neighbors {
            if let Some(status) = self.seat_map.get(neighbor) {
                if *status { occupied_neighbors_count += 1; }
            }
        }
        // make this loop functional
        occupied_neighbors_count
    }
}

fn create_map(filepath: &str) -> HashMap<Coord, bool> {
    let mut seats = HashMap::<Coord, bool>::new();
    if let Ok(lines) = read_lines(filepath) {
        for (row, line) in lines.enumerate() {
            if let Ok(row_symbols) = line {
                for(column, symbol) in row_symbols.chars().enumerate() {
                    if let 'L' = symbol {
                        seats.insert(Coord(row, column), false);
                    } else if symbol == '#' {
                        seats.insert(Coord(row, column), true);
                    }
                }
            }
        }
    }
    seats
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod coord_tests {
    
    use super::*;

    #[test]
    fn test_coords_can_be_correctly_created() {
        let location = Coord(0, 1);
        assert_eq!(Coord(0, 1), location);
    }

    #[test]
    fn get_neighbors_correctly_for_middle_location() {
        let location = Coord(1, 1);
        let mut expected_neighbors = HashSet::from([Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 0), Coord(1, 2), Coord(2, 0), Coord(2, 1), Coord(2, 2)]);
        assert_eq!(expected_neighbors, location.get_neighbors());
    }

    #[test]
    fn get_neighbors_correctly_for_edge_location() {
        let location = Coord(0, 1);
        let mut expected_neighbors = HashSet::from([Coord(0, 0), Coord(0, 2), Coord(1, 0), Coord(1, 1), Coord(1, 2)]);
        assert_eq!(expected_neighbors, location.get_neighbors());
    }

    #[test]
    fn get_neighbors_correctly_for_corner_location() {
        let location = Coord(0, 0);
        let mut expected_neighbors = HashSet::from([Coord(0, 1), Coord(1, 0), Coord(1, 1)]);
        assert_eq!(expected_neighbors, location.get_neighbors());
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waiting_area_correctly_created() {
        let waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
        assert!(waiting_area.is_seat(&Coord(0, 0)));
        assert!(waiting_area.is_seat(&Coord(0, 2)));
        assert!(waiting_area.is_seat(&Coord(0, 3)));
        assert!(waiting_area.is_seat(&Coord(1, 0)));
        assert!(waiting_area.is_seat(&Coord(2, 0)));
        assert!(waiting_area.is_seat(&Coord(9, 9)));
        assert_eq!(false, waiting_area.is_seat(&Coord(0, 1)));
        assert_eq!(false, waiting_area.is_seat(&Coord(2, 1)));
    }

    #[test]
    fn test_a_seat_is_occupied() {
        let waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
        assert_eq!(false, waiting_area.is_occupied(&Coord(0, 0)));
        assert_eq!(false, waiting_area.is_occupied(&Coord(0, 2)));
        assert_eq!(false, waiting_area.is_occupied(&Coord(0, 1)));
        assert_eq!(false, waiting_area.is_occupied(&Coord(9, 9)));
    }

    #[test]
    fn rule_for_occupied_seats_work_normally() {
        let mut occupied_waiting_area = WaitingArea::new("test_data/all_seats_occupied.txt");
        assert_eq!(true, occupied_waiting_area.apply_rule_for_occupied_seat_on(&Coord(0, 0)));
        assert_eq!(false, occupied_waiting_area.apply_rule_for_occupied_seat_on(&Coord(0, 2)));
    }

    #[test]
    fn rule_for_empty_seats_work_normally() {
        let mut empty_waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
        assert_eq!(true, empty_waiting_area.apply_rule_for_empty_seat_on(&Coord(0, 0)));
        assert_eq!(true, empty_waiting_area.apply_rule_for_empty_seat_on(&Coord(0, 2)));
    }

    #[test]
    fn can_update_entire_map() {
        // let mut waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
        // waiting_area.update();
        // assert!(waiting_area.is_occupied(&Coord(0, 0)));
        // assert!(waiting_area.is_occupied(&Coord(0, 2)));
        // assert!(waiting_area.is_occupied(&Coord(9, 9)));
        // assert_eq!(false, waiting_area.is_occupied(&Coord(0, 1)));
    }
}