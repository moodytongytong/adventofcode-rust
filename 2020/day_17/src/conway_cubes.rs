use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(isize, isize, isize);

// n = len(pool)
// for indices in itertools.product(range(-1,2), repeat=n):

// let tuple = get_tuples([]) -> [(-1), (0), (1)]
// get_tuples([(-1), (0), (1)]) -> [(-1,-1), (0,-1), (1,-1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]

impl Neighbors for Coord {
    fn get_neighbors(&self) -> HashSet<Coord> {
        let mut neighbors = HashSet::<Coord>::new();
        let deltas : [(i8, i8, i8); 26] = [
            (-1, -1, -1),(-1, -1, 0),(-1, -1, 1),
            (-1, 0, -1),(-1, 0, 0),(-1, 0, 1),
            (-1, 1, -1),(-1, 1, 0),(-1, 1, 1),
            (0, -1, -1),(0, -1, 0),(0, -1, 1),
            (0, 0, -1),(0, 0, 1),
            (0, 1, -1),(0, 1, 0),(0, 1, 1),
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

trait Neighbors {
    fn get_neighbors(&self) -> HashSet<Coord>;
}

pub struct World {
    active_cubes: RefCell<HashSet<Coord>>,
}

impl World {
    pub fn count_active(&self) -> usize {
        self.active_cubes.borrow().iter().count()
    }

    fn apply_rule_on(&self, position: Coord) -> bool {
        if self.active_cubes.borrow().contains(&position) {
            return self.active_rule_on(position);
        } else {
            return self.inactive_rule_on(position);
        }
    }

    fn active_rule_on(&self, position: Coord) -> bool {
        let active_neighbor_count = self.count_active_neighbors_of(position);
        return active_neighbor_count == 2 || active_neighbor_count == 3;
    }

    fn inactive_rule_on(&self, position: Coord) -> bool {
        return self.count_active_neighbors_of(position) == 3;
    }

    fn count_active_neighbors_of(&self, position: Coord) -> usize {
        position.get_neighbors().iter().filter(
            |&neighbor| self.active_cubes.borrow().contains(neighbor) ).count()
    }

    fn update(&self) {
        let mut new_cubes = HashSet::<Coord>::new();
        for cube in self.active_cubes.borrow().iter() {
            new_cubes = new_cubes.union(&cube.get_neighbors()).copied().collect::<HashSet<_>>();
            new_cubes.insert(*cube);
        }
        let mut new_active_cubes = HashSet::<Coord>::new();
        for cube in new_cubes {
            if self.apply_rule_on(cube) {
                new_active_cubes.insert(cube);
            }
        }
        self.active_cubes.replace(new_active_cubes);
    }

    pub fn update_times(&self, count: usize) {
        for _ in 0..count { self.update(); }
    }
}

pub fn create_world_from(filepath: &str) -> World {
    let mut active_cubes = HashSet::<Coord>::new();
    if let Ok(lines) = read_lines(filepath) {
        let mut y = 0;
        for line in lines {
            let mut x = 0;
            if let Ok(symbols) = line {
                for symbol in symbols.chars(){
                    let position = Coord(x, y, 0);
                    if symbol == '#' {
                        active_cubes.insert(position);
                    }
                    x += 1;
                }
            }
            y += 1;
        }
    }
    World { active_cubes: RefCell::new(active_cubes) }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn get_tuples( :HashSet<>) -> HashSet {
    
// }

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn get_neighbors_correctly() {
        let cube = Coord(0, 0, 0);
        let expected_neighbors = HashSet::from([
            Coord(-1, -1, -1),Coord(-1, -1, 0),Coord(-1, -1, 1),
            Coord(-1, 0, -1),Coord(-1, 0, 0),Coord(-1, 0, 1),
            Coord(-1, 1, -1),Coord(-1, 1, 0),Coord(-1, 1, 1),
            Coord(0, -1, -1),Coord(0, -1, 0),Coord(0, -1, 1),
            Coord(0, 0, -1),Coord(0, 0, 1),
            Coord(0, 1, -1),Coord(0, 1, 0), Coord(0, 1, 1),
            Coord(1, -1, -1),Coord(1, -1, 0),Coord(1, -1, 1),
            Coord(1, 0, -1),Coord(1, 0, 0),Coord(1, 0, 1),
            Coord(1, 1, -1),Coord(1, 1, 0),Coord(1, 1, 1)
            ]);
        assert_eq!(expected_neighbors, cube.get_neighbors());
    }

    #[test]
    fn world_correctly_created_from_input() {
        let world = create_world_from("test_data/test1.txt");
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
        assert_eq!(false, world.apply_rule_on(Coord(1, 0, 0)));
    }

    #[test]
    fn inactive_node_rule_correctly_applies() {
        let world = create_world_from("test_data/test1.txt");
        assert_eq!(true, world.apply_rule_on(Coord(0, 1, 0)));
        assert_eq!(false, world.apply_rule_on(Coord(0, 0, 0)));
    }

    #[test]
    fn test_world_could_update_correctly() {
        let world = create_world_from("test_data/test1.txt");
        world.update();
        assert_eq!(11, world.count_active());
        world.update();
        assert_eq!(21, world.count_active());
        world.update();
        assert_eq!(38, world.count_active());
    }

    #[test]
    fn test_world_correctly_updates_for_6_cycles() {
        let world = create_world_from("test_data/test1.txt");
        world.update_times(6);
        assert_eq!(112, world.count_active());
    }

    // #[test]
    // fn get_tuples_correctly() {
    //     let tuple = get_tuples(HashSet::<isize>::new());
    //     let expected_result = HashSet::from([
    //         (-1), (0), (1)
    //         ]);
    //     assert_eq!(expected_result, tuple);
    // }
}