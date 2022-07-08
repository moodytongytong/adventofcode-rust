use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord (usize, usize);

fn create_map(filepath: &str) -> HashSet<Coord> {
    let mut seats = HashSet::<Coord>::new();
    if let Ok(lines) = read_lines(filepath) {
        for (row, line) in lines.enumerate() {
            if let Ok(row_symbols) = line {
                for(column, symbol) in row_symbols.chars().enumerate() {
                    if let 'L' = symbol {
                        seats.insert(Coord(row, column));
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
    
    use super::Coord;

    #[test]
    fn test_coords_can_be_correctly_created() {
        let location = Coord(0, 1);
        assert_eq!(Coord(0, 1), location);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_seats_correctly_created() {
        let empty_seats = create_map("test_data/test1.txt");
        assert!(empty_seats.contains(&Coord(0, 0)));
        assert!(empty_seats.contains(&Coord(0, 2)));
        assert!(empty_seats.contains(&Coord(0, 3)));
        assert!(empty_seats.contains(&Coord(1, 0)));
        assert!(empty_seats.contains(&Coord(2, 0)));
        assert!(empty_seats.contains(&Coord(9, 9)));
    }

    // need to think about how to continue
    // do I create another set with empty seats or another set with occupied seats
    // should it be functional or object oriented
}