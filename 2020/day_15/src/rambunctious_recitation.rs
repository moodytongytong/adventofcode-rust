use std::collections::HashMap;

pub fn find_turn(final_turn: usize, starting_numbers: &str) -> usize {
    let mut current_number = 0;
    let mut current_turn = 0;
    let mut num_to_last_turn = HashMap::<usize, usize>::new();
    for word in starting_numbers.split(",") {
        if let Ok(number) = word.parse::<usize>() {
            current_turn += 1;
            current_number = number;
            num_to_last_turn.insert(number, current_turn);
        }
    }
    let starting_turn = current_turn;
    for current_turn in starting_turn..final_turn {
        if let Some(last_appeared) = num_to_last_turn.get(&current_number) {
            let next_number = current_turn - last_appeared;
            num_to_last_turn.insert(current_number, current_turn);
            current_number = next_number;
            
        } else {
            num_to_last_turn.insert(current_number, current_turn);
            current_number = 0;
        }
    }
    current_number
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn find_next_number_correctly() {
        let turn_4_number = find_turn(4, "0,3,6");
        assert_eq!(0, turn_4_number);

        let turn_5_number = find_turn(5, "0,3,6");
        assert_eq!(3, turn_5_number);

        let turn_6_number = find_turn(6, "0,3,6");
        assert_eq!(3, turn_6_number);

        let turn_7_number = find_turn(7, "0,3,6");
        assert_eq!(1, turn_7_number);

        let turn_8_number = find_turn(8, "0,3,6");
        assert_eq!(0, turn_8_number);

        let turn_9_number = find_turn(9, "0,3,6");
        assert_eq!(4, turn_9_number);

        let turn_10_number = find_turn(10, "0,3,6");
        assert_eq!(0, turn_10_number);

        let turn_2020_number = find_turn(2020, "0,3,6");
        assert_eq!(436, turn_2020_number);

        let turn_30000000_number = find_turn(30000000, "0,3,6");
        assert_eq!(175594, turn_30000000_number);
        // TAKES TOO LONG TO COMPLETE
    }

    #[test]
    fn test_example_132_works() {
        let turn_2020_number = find_turn(2020, "1,3,2");
        assert_eq!(1, turn_2020_number);
    }

    #[test]
    fn test_example_213_works() {
        let turn_2020_number = find_turn(2020, "2,1,3");
        assert_eq!(10, turn_2020_number);
    }

    #[test]
    fn test_example_123_works() {
        let turn_2020_number = find_turn(2020, "1,2,3");
        assert_eq!(27, turn_2020_number);
    }

    #[test]
    fn test_example_312_works() {
        let turn_2020_number = find_turn(2020, "3,1,2");
        assert_eq!(1836, turn_2020_number);
    }
}