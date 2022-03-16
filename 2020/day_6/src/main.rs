mod custom_customs;
use custom_customs::{
    create_data_by_groups,
    find_sum_of_counts,
    find_pt1_count_from_group,
};
use std::io;

fn main() -> Result<(), io::Error>{
    let data = create_data_by_groups("test_data/input.txt")?;
    println!("The sum of counts from the flight is {}", find_sum_of_counts(data, &find_pt1_count_from_group));
    Ok(())
}
