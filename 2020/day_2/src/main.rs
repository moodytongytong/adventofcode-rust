mod wrong_password_finder;
use wrong_password_finder::{new_records,
    find_number_of_valid_records,
    policy_pt1,
    policy_pt2
};

fn main() {
    let records = new_records("test_data/input.txt");
    println!("part 1 result is {}", find_number_of_valid_records(&records, &policy_pt1));
    println!("part 2 result is {}", find_number_of_valid_records(&records, &policy_pt2));
}
