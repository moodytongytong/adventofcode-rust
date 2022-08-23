mod rain_risk;
use rain_risk::Ship;
use rain_risk::read_instructions;

fn main() {
    let mut ship = Ship::new();
    let instructions = read_instructions("test_data/input.txt");
    println!("The solution to part 1 is {}", ship.find_part1_distance_from_intructions(&instructions));
    ship.reset();
    println!("The solution to part 2 is {}", ship.find_part2_distance_from_intructions(&instructions));
}
