mod forrest;
use forrest::Map;
use forrest::part1;
use forrest::part2;

fn main() {
    let map = Map::new("test_data/input.txt");
    println!("part 1 solution is {}", part1(&map));
    println!("part 2 solution is {}", part2(&map));
}
