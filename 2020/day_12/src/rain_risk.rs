use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, PartialOrd, Ord)]
struct Coord (isize, isize);

impl Coord {
    fn distance_from(self, target: Coord) -> usize {
        (self.0 - target.0).abs() as usize + (self.1 - target.1).abs() as usize
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum Direction {
    E,
    N,
    W,
    S,
}

pub struct Ship {
    direction: Direction,
    location: Coord,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            direction: Direction::E,
            location: Coord(0, 0),
        }
    }

    fn shift(&mut self, instruction: &str) {
        let action = instruction.chars().nth(0).unwrap();
        let value = instruction[1..].parse::<usize>().unwrap();
        match action {
            'L' => self.turn_left(value),
            'R' => self.turn_right(value),
            'E' => self.shift_east(value),
            'N' => self.shift_north(value),
            'W' => self.shift_west(value),
            'S' => self.shift_south(value),
            _ => self.shift_forward(value),
        }
    }

    fn turn_left(&mut self, degree: usize) {
        let turn_value = degree / 90;
        let direction_index = (turn_value as i8 + self.direction as i8).rem_euclid(4) as u8;
        self.direction = convert_to_direction(direction_index);
    }

    fn turn_right(&mut self, degree: usize) {
        let turn_value = degree / 90;
        let direction_index = (self.direction as i8 - turn_value as i8).rem_euclid(4) as u8;
        self.direction = convert_to_direction(direction_index);
    }

    fn shift_east(&mut self, distance: usize) {
        self.location.0 += distance as isize;
    }

    fn shift_north(&mut self, distance: usize) {
        self.location.1 += distance as isize;
    }

    fn shift_west(&mut self, distance: usize) {
        self.location.0 -= distance as isize;
    }

    fn shift_south(&mut self, distance: usize) {
        self.location.1 -= distance as isize;
    }

    fn shift_forward(&mut self, distance: usize) {
        match self.direction {
            Direction::E => self.shift_east(distance),
            Direction::N => self.shift_north(distance),
            Direction::W => self.shift_west(distance),
            _ => self.shift_south(distance),
        }
    }

    pub fn find_distance_from_intructions(&mut self, instruction_filepath: &str) -> usize {
        let mut instructions = Vec::<String>::new();
        if let Ok(lines) = read_lines(instruction_filepath) {
            for line in lines {
                if let Ok(instruction) = line {
                    instructions.push(instruction);
                }
            }
        }
        for instruction in instructions {
            self.shift(&instruction[..]);
        }
        self.location.distance_from(Coord(0, 0))
    }
}

fn convert_to_direction(direction_index: u8) -> Direction {
    match direction_index{
        0 => Direction::E,
        1 => Direction::N,
        2 => Direction::W,
        _ => Direction::S,
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
    fn a_ship_is_correctly_created_with_default_values() {
        let ship = Ship::new();
        assert_eq!(Direction::E, ship.direction);
        assert_eq!(Coord(0, 0), ship.location);
    }

    #[test]
    fn instructions_to_turn_are_correctly_processed() {
        let mut ship = Ship::new();
        // left turns
        ship.shift("L90");
        assert_eq!(Direction::N, ship.direction);
        ship.shift("L180");
        assert_eq!(Direction::S, ship.direction);
        ship.shift("L270");
        assert_eq!(Direction::W, ship.direction);
        ship.shift("L90");
        assert_eq!(Direction::S, ship.direction);
        ship.shift("L180");
        assert_eq!(Direction::N, ship.direction);
        ship.shift("L270");
        assert_eq!(Direction::E, ship.direction);
        ship.shift("L90");
        assert_eq!(Direction::N, ship.direction);
        ship.shift("L90");
        assert_eq!(Direction::W, ship.direction);
        ship.shift("L180");
        assert_eq!(Direction::E, ship.direction);
        ship.shift("L270");
        assert_eq!(Direction::S, ship.direction);
        ship.shift("L90");
        assert_eq!(Direction::E, ship.direction);

        // right turns
        ship.shift("R90");
        assert_eq!(Direction::S, ship.direction);
        ship.shift("R180");
        assert_eq!(Direction::N, ship.direction);
        ship.shift("R270");
        assert_eq!(Direction::W, ship.direction);
        assert_eq!(Coord(0, 0), ship.location);
    }

    #[test]
    fn instructions_to_move_are_correctly_processed() {
        let mut ship = Ship::new();
        ship.shift("F1");
        assert_eq!(Coord(1, 0), ship.location);
        ship.shift("N2");
        assert_eq!(Coord(1, 2), ship.location);
        ship.shift("E3");
        assert_eq!(Coord(4, 2), ship.location);
        ship.shift("S4");
        assert_eq!(Coord(4, -2), ship.location);
        ship.shift("W5");
        assert_eq!(Coord(-1, -2), ship.location);
    }

    #[test]
    fn find_manhattan_distance_from_start() {
        let mut ship = Ship::new();
        let movement = ship.find_distance_from_intructions("test_data/test1.txt");
        assert_eq!(25, movement);
    }
}