use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{
    HashMap, HashSet
};
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(isize, isize, isize);

impl Coord {
    fn get_neighbors(&self) -> HashSet<Coord> {
        let mut neighbors = HashSet::<Coord>::new();
        let deltas : [(i8, i8, i8); 26] = [
            (-1, -1, -1),(-1, -1, 0),(-1, -1, 1),
            (-1, 0, -1),(-1, 0, 0),(-1, 0, 1),
            (-1, 1, -1),(-1, 1, 0),(-1, 0, 1),
            (0, -1, -1),(0, -1, 0),(0, -1, 1),
            (0, 0, -1),(0, 0, 1),
            (0, 1, -1),(0, 1, 0),(0, 0, 1),
            (1, -1, -1),(1, -1, 0),(1, -1, 1),
            (1, 0, -1),(1, 0, 0),(1, 0, 1),
            (1, 1, -1),(1, 1, 0),(1, 1, 1)
        ];
        
        for delta in deltas {
            let x = delta.0 as isize + self.0;
            let y = delta.1 as isize + self.1;
            let z = delta.2 as isize + self.2;
            neighbors.insert(Coord(x, y, z));
        }

        neighbors
    }
}

// #[derive(Debug, PartialEq, Eq, Hash)]
// struct Cube {
//     position: Coord,
//     is_active: bool,
// }

struct World {
    cubes: RefCell<HashMap<Coord, bool>>,
}

impl World {
    fn count_active(&self) -> usize {
        self.cubes.borrow().values().filter(|&is_active| *is_active).count()
    }

    fn apply_rule_on(&self, position: Coord) -> bool {
        if let Some(state) = self.cubes.borrow().get(&position) {
            if *state { return self.active_rule_on(position); }
            else { return self.inactive_rule_on(position); }
        } else {
            return self.inactive_rule_on(position);
        }
    }

    fn active_rule_on(&self, position: Coord) -> bool {
        true
    }

    fn inactive_rule_on(&self, position: Coord) -> bool {
        true
    }

    fn count_active_neighbors_of(&self, position: Coord) -> usize {
        position.get_neighbors().iter().filter(
            |&neighbor| self.cubes.borrow().get(neighbor) != None && 
            self.cubes.borrow()[neighbor] ).count()
    }
}



fn create_world_from(filepath: &str) -> World {
    let mut cubes = HashMap::<Coord, bool>::new();
    if let Ok(lines) = read_lines(filepath) {
        let mut x = 0;
        for line in lines {
            let mut y = 0;
            if let Ok(symbols) = line {
                // println!("the variable symbols is {}", symbols);
                for symbol in symbols.chars(){
                    let position = Coord(x, y, 0);
                    // println!("The position is {:?}", position);
                    if symbol == '#' {
                        cubes.insert(position, true);
                    } else {
                        cubes.insert(position, false);
                    }
                    y += 1;
                }
            }
            x += 1;
        }
    }
    World { cubes: RefCell::new(cubes) }
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
    fn get_neighbors_correctly() {
        let cube = Coord(0, 0, 0);
        let expected_neighbors = HashSet::from([
            Coord(-1, -1, -1),Coord(-1, -1, 0),Coord(-1, -1, 1),
            Coord(-1, 0, -1),Coord(-1, 0, 0),Coord(-1, 0, 1),
            Coord(-1, 1, -1),Coord(-1, 1, 0),Coord(-1, 0, 1),
            Coord(0, -1, -1),Coord(0, -1, 0),Coord(0, -1, 1),
            Coord(0, 0, -1),Coord(0, 0, 1),
            Coord(0, 1, -1),Coord(0, 1, 0), Coord(0, 0, 1),
            Coord(1, -1, -1),Coord(1, -1, 0),Coord(1, -1, 1),
            Coord(1, 0, -1),Coord(1, 0, 0),Coord(1, 0, 1),
            Coord(1, 1, -1),Coord(1, 1, 0),Coord(1, 1, 1)
            ]);
        assert_eq!(expected_neighbors, cube.get_neighbors());
    }

    #[test]
    fn world_correctly_created_from_input() {
        let world = create_world_from("test_data/test1.txt");
        assert_eq!(9, world.cubes.borrow().len());
        assert_eq!(5, world.count_active());
    }

    #[test]
    fn count_active_neighbors_correctly() {
        let world = create_world_from("test_data/test1.txt");
        assert_eq!(5, world.count_active_neighbors_of(Coord(1,1,0)));
    }

    #[test]
    fn active_node_rule_correctly_applies() {
        let world = create_world_from("test_data/test1.txt");
        assert_eq!(true, world.apply_rule_on(Coord(2, 1, 0)));
    }

    #[test]
    fn inactive_node_rule_correctly_applies() {

    }

    // need to find out how counting is done exactly


}