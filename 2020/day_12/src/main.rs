mod rain_risk;
use rain_risk::Ship;

fn main() {
    let mut ship = Ship::new();
    println!("The solution to part 1 is {}", ship.find_distance_from_intructions("test_data/input.txt"));
}
