mod conway_cubes;
use conway_cubes::create_world_from;

fn main() {
    let world = create_world_from("test_data/input.txt");
    world.update_times(6);
    println!("The number of active cubes after 6 cycles is {}", world.count_active());
}
