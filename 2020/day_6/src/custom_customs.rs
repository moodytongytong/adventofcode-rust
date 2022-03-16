use std::fs::read_to_string;
use std::io;
use std::collections::HashSet;

pub fn create_data_by_groups(filepath: &str) -> Result<Vec<String>, io::Error> {
    let raw_data = read_to_string(filepath)?;
    Ok(raw_data.split("\n\n").map(|item| item.to_string()).collect())
}

pub fn find_sum_of_counts(data: Vec<String>, rule: &dyn Fn(&str) -> usize) -> usize {
    data.iter().map(|group_data| rule(group_data)).sum()
}

pub fn find_pt1_count_from_group(data: &str) -> usize {
    let mut unique_chars = HashSet::<char>::new();
    for c in data.chars() {
        unique_chars.insert(c);
    }
    unique_chars.remove(&'\n');
    unique_chars.len()
}

// pub fn find_pt2_count_from_group(data: &str) -> usize {
//     3
// }


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn find_pt1_count_from_a_group() {
        let group1_data = "abcx\nabcy\nabcz";
        assert_eq!(6, find_pt1_count_from_group(group1_data));

        let group2_data = "abc";
        assert_eq!(3, find_pt1_count_from_group(group2_data));

        let group2_data = "a\nb\nc";
        assert_eq!(3, find_pt1_count_from_group(group2_data));
    }

    // fn find_pt2_count_from_a_group() {
    //     let group1_data = "abcx\nabcy\nabcz";
    //     assert_eq!(3, find_pt2_count_from_group(group1_data));
    // }

    #[test]
    fn data_correctly_created() -> Result<(), io::Error>{
        let data = create_data_by_groups("test_data/test1.txt")?;
        assert_eq!(5, data.len());
        Ok(())
    }

    #[test]
    fn find_sum_of_all_counts_from_plane() {
        let data = create_data_by_groups("test_data/test1.txt").unwrap();
        assert_eq!(11, find_sum_of_counts(data, &find_pt1_count_from_group));
    }


}