use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, PartialOrd, Ord)]
struct Coord (isize, isize);

struct Ship {
    direction: Direction,
    direction_index: i8,
    location: Coord,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: Direction::E,
            direction_index: 0,
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
        let new_direction_index = (turn_value as i8 + self.direction_index) % 4;
        self.direction_index = new_direction_index;
        self.update_direction();
    }

    fn turn_right(&mut self, degree: usize) {
        let turn_value = degree / 90;
        let new_direction_index = (self.direction_index - turn_value as i8) % 4;
        self.direction_index = new_direction_index;
        self.update_direction();
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

    fn update_direction(&mut self) {
        match self.direction_index {
            0 => self.direction = Direction::E,
            1 => self.direction = Direction::N,
            2 => self.direction = Direction::W,
            _ => self.direction = Direction::S,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    E,
    N,
    W,
    S,
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
        ship.shift("L90");
        assert_eq!(Direction::N, ship.direction);
        ship.shift("L180");
        assert_eq!(Direction::S, ship.direction);
        ship.shift("L270");
        assert_eq!(Direction::W, ship.direction);
        ship.shift("R90");
        assert_eq!(Direction::N, ship.direction);
        ship.shift("R180");
        assert_eq!(Direction::S, ship.direction);
        ship.shift("R270");
        assert_eq!(Direction::E, ship.direction);
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
}