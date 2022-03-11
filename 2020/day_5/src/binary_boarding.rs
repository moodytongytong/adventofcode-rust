use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_row(partition_code: &str) -> usize {
    let row_code = &partition_code[0..7];
    let mut row = 0;
    let mut change = 128;
    for letter in row_code.chars() {
        change /= 2;
        if let 'B' = letter {
            row += change;
        }
    }
    row
}

fn find_column(partition_code: &str) -> usize {
    let column_code = &partition_code[7..];
    let mut column = 0;
    let mut change = 8;
    for letter in column_code.chars() {
        change /= 2;
        if let 'R' = letter {
            column += change;
        }
    }
    column
}

fn find_seat_id(partition_code: &str) -> usize {
    find_row(&partition_code) * 8 + find_column(&partition_code)
}

pub fn create_codes_from_path(filepath: &str) -> HashSet<usize>{
    let mut all_seat_codes = HashSet::<usize>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(code) = line {
                all_seat_codes.insert(find_seat_id(&code));
            }
        }
    }
    all_seat_codes
}

pub fn find_highest_id_from(seat_codes: &HashSet<usize>) -> usize {
    *seat_codes.iter().max().unwrap()
}

pub fn find_missing_id_from(seat_codes: &HashSet<usize>) -> Result<usize, bool> {
    for n in 8..1024 {
        if !(seat_codes.contains(&n)) && seat_codes.contains(&(n-1)) && seat_codes.contains(&(n+1)) {
            return Ok(n);
        }
    }
    Err(false)
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn row_correctly_returned_with_partition_code() {
        let seat_1 = "FBFBBFFRLR";
        assert_eq!(44, find_row(&seat_1));

        let seat_2 = "BFFFBBFRRR";
        assert_eq!(70, find_row(&seat_2));

        let seat_3 = "FFFBBBFRRR";
        assert_eq!(14, find_row(&seat_3));

        let seat_4 = "BBFFBBFRLL";
        assert_eq!(102, find_row(&seat_4));

    }

    #[test]
    fn column_correctly_returned_with_partition_code() {
        let seat_1 = "FBFBBFFRLR";
        assert_eq!(5, find_column(&seat_1));

        let seat_2 = "BFFFBBFRRR";
        assert_eq!(7, find_column(&seat_2));

        let seat_3 = "FFFBBBFRRR";
        assert_eq!(7, find_column(&seat_3));

        let seat_4 = "BBFFBBFRLL";
        assert_eq!(4, find_column(&seat_4));
    }

    #[test]
    fn seat_id_correctly_returned_with_partition_code() {
        let seat_1 = "FBFBBFFRLR";
        assert_eq!(357, find_seat_id(&seat_1));

        let seat_2 = "BFFFBBFRRR";
        assert_eq!(567, find_seat_id(&seat_2));

        let seat_3 = "FFFBBBFRRR";
        assert_eq!(119, find_seat_id(&seat_3));

        let seat_4 = "BBFFBBFRLL";
        assert_eq!(820, find_seat_id(&seat_4));
    }

    #[test]
    fn highest_id_correctly_found() {
        let seat_codes = create_codes_from_path("test_data/test1.txt");
        assert_eq!(820, find_highest_id_from(&seat_codes));
    }
}