mod binary_boarding;
use binary_boarding::{
    find_highest_id_from,
    create_codes_from_path,
    find_missing_id_from,
};

fn main() -> Result<(), bool>{
    let codes = create_codes_from_path("test_data/input.txt");
    println!("The highet seat ID is {}", find_highest_id_from(&codes));
    println!("The missing seat ID is {}", find_missing_id_from(&codes)?);
    Ok(())
}
