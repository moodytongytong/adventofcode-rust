use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{
    HashMap,
    HashSet,
    BTreeMap,
};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, PartialOrd, Ord)]
struct Coord (usize, usize);

impl Coord {
    fn get_neighbors(&self) -> HashSet<Coord> {
        let mut neighbors = HashSet::new();
        let deltas : [(i8, i8); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
        for delta in deltas {
            let x = self.0 as i8 + delta.0;
            let y = self.1 as i8 + delta.1;
            if x >= 0 && y >= 0 {
                neighbors.insert(Coord(x as usize, y as usize));
            }
        }
        neighbors
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
        }
        false
    }

    fn update(&mut self) {
        let mut new_map = HashMap::<Coord, bool>::new();
        for location in self.seat_map.keys() {
            if let Ok(status) = self.apply_rule(location) {
                new_map.insert(*location, status);
            }
        }
        self.seat_map = new_map;
    }

    fn apply_rule(&self, location: &Coord) -> Result<bool, String> {
        let num_neighbours = self.find_number_of_occupied_neighbors(location);
        if let Some(status) = self.seat_map.get(location) {
            if *status { return Ok(num_neighbours < 4); }
            else { return Ok(num_neighbours == 0); }
        }
        Err("Invalid data".to_string())
    }

    fn find_number_of_occupied_neighbors(&self, location: &Coord) -> u8 {
        let neighbors = location.get_neighbors();
        neighbors.iter().filter(|&neighbor| self.is_occupied(neighbor) ).count() as u8
    }

    // fn run_until_stabilizes(&mut self) {
    //     let mut previous_map = &self.seat_map;
    //     self.update();
    //     while !previous_map.eq(&self.seat_map) {
    //         self.update();
    //         previous_map = &self.seat_map;
    //         // LOOK FOR A WAY TO CHECK EQUALITY BETWEEN TWO MAPS
    //         // IF NOT POSSIBLE, I MIGHT NEED TO LOOK FOR WAYS TO FIND OUT WHETHER A MAP HAD BEEN UPDATED
    //     }
    // }

    fn find_occupied_seat_count(&self) -> usize {
        self.seat_map.values().filter(|&status| *status ).count()
    }
}

// could this be simplified?
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
        let expected_neighbors = HashSet::from([Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 0), Coord(1, 2), Coord(2, 0), Coord(2, 1), Coord(2, 2)]);
        assert_eq!(expected_neighbors, location.get_neighbors());
    }

    #[test]
    fn get_neighbors_correctly_for_edge_location() {
        let location = Coord(0, 1);
        let expected_neighbors = HashSet::from([Coord(0, 0), Coord(0, 2), Coord(1, 0), Coord(1, 1), Coord(1, 2)]);
        assert_eq!(expected_neighbors, location.get_neighbors());
    }

    #[test]
    fn get_neighbors_correctly_for_corner_location() {
        let location = Coord(0, 0);
        let expected_neighbors = HashSet::from([Coord(0, 1), Coord(1, 0), Coord(1, 1)]);
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
        let occupied_waiting_area = WaitingArea::new("test_data/all_seats_occupied.txt");
        assert_eq!(Ok(true), occupied_waiting_area.apply_rule(&Coord(0, 0)));
        assert_eq!(Ok(false), occupied_waiting_area.apply_rule(&Coord(0, 2)));
    }

    #[test]
    fn rule_for_empty_seats_work_normally() {
        let empty_waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
        assert_eq!(Ok(true), empty_waiting_area.apply_rule(&Coord(0, 0)));
        assert_eq!(Ok(true), empty_waiting_area.apply_rule(&Coord(0, 2)));
    }

    #[test]
    fn can_update_entire_map() {
        let mut waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
        waiting_area.update();
        assert!(waiting_area.is_occupied(&Coord(0, 0)));
        assert!(waiting_area.is_occupied(&Coord(0, 2)));
        assert!(waiting_area.is_occupied(&Coord(9, 9)));
        assert_eq!(false, waiting_area.is_occupied(&Coord(0, 1)));

        waiting_area.update();
        assert!(waiting_area.is_occupied(&Coord(0, 0)));
        assert_eq!(false, waiting_area.is_occupied(&Coord(0, 2)));
        assert!(waiting_area.is_occupied(&Coord(9, 9)));
        assert_eq!(false, waiting_area.is_occupied(&Coord(0, 1)));
    }

    // #[test]
    // fn correctly_finds_occupied_seat_count_after_map_stabalizes() {
    //     let mut waiting_area = WaitingArea::new("test_data/all_seats_empty.txt");
    //     waiting_area.run_until_stabilizes();
    //     assert_eq!(37, waiting_area.find_occupied_seat_count());
    // }
}