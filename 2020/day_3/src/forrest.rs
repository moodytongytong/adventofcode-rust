use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub struct Map {
    height: usize,
    width: usize,
    tree_locations: HashSet<Coord>,
}

impl Map {
    pub fn new(filepath: &str) -> Self {
        let mut tree_locations: HashSet<Coord> = HashSet::new();
        let mut y = 0;
        let mut x = 0;
        if let Ok(lines) = read_lines(filepath) {
            for line in lines {
                if let Ok(info) = line {
                    x = 0;
                    for symbol in info.chars() {
                        if symbol == '#' {
                            tree_locations.insert(Coord::new(x, y));
                        }
                        x += 1;
                    }
                    y += 1;
                }
            }
        }
        Map {
            height: y,
            width: x,
            tree_locations,
        }
    }

    pub fn find_number_of_trees_in_journey(&self, slope: (usize, usize)) -> usize {
        let mut count = 0;
        let mut toboggan = Coord::new(0, 0);
        while toboggan.y < self.height {
            toboggan = self.translate(toboggan, slope);
            if self.tree_locations.contains(&toboggan) {
                count += 1;
            }
        }
        count
    }

    fn translate(&self, toboggan: Coord, slope: (usize, usize)) -> Coord {
        Coord::new(
            (toboggan.x + slope.0) % self.width,
            toboggan.y + slope.1
        )
    }
 }

#[derive(Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord {x, y}
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1(map: &Map) -> usize {
    map.find_number_of_trees_in_journey((3,1))
}

pub fn part2(map: &Map) -> usize {
    let slopes = [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().fold(1, |product, &slope| product * map.find_number_of_trees_in_journey(slope))
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_map_correctly_read_in() {
        let map = Map::new("test_data/test1.txt");
        assert_eq!(map.height, 11);
        assert_eq!(map.width, 11);
        assert!(map.tree_locations.contains(&Coord::new(2, 0)));
        assert!(map.tree_locations.contains(&Coord::new(0, 1)));
        assert!(map.tree_locations.contains(&Coord::new(10, 10)));
    }

    #[test]
    fn test_coord_correctly_created() {
        let point = Coord::new(1, 2);
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn trees_encountered_in_journey_correctly_counted () {
        let map = Map::new("test_data/test1.txt");
        assert_eq!(map.find_number_of_trees_in_journey((3, 1)), 7);
    }

    #[test]
    fn trees_corretly_counted_for_another_slope() {
        let map = Map::new("test_data/test1.txt");
        assert_eq!(map.find_number_of_trees_in_journey((1, 1)), 2);
        assert_eq!(map.find_number_of_trees_in_journey((5, 1)), 3);
        assert_eq!(map.find_number_of_trees_in_journey((7, 1)), 4);
        assert_eq!(map.find_number_of_trees_in_journey((1, 2)), 2);
    }
}