mod rambunctious_recitation;
use rambunctious_recitation::find_turn;

fn main() {
    println!("The answer to part 1 is {}", find_turn(2020, "7,12,1,0,16,2"));
    println!("The answer to part 2 is {}", find_turn(30000000, "7,12,1,0,16,2"));
}
