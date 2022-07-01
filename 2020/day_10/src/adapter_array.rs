use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
<<<<<<< HEAD
use std::collections::{
    HashMap,
    HashSet,
};

pub fn create_ascending_adapters(filepath: &str) -> Vec<usize> {
=======

pub fn create_adapters(filepath: &str) -> Vec<usize> {
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
    let mut adapters = Vec::<usize>::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(number) = line {
                adapters.push(number.parse().unwrap());
            } 
        }
    }
<<<<<<< HEAD
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);
    adapters
}

pub fn find_differences_of_1_and_3_jolts_from(adapters: &Vec<usize>) -> (usize, usize) {
=======
    adapters
}

pub fn find_differences_of_1_and_3_jolts_from(mut adapters: Vec<usize>) -> (usize, usize) {
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
    let mut one_difference_count = 0;
    let mut three_difference_count = 0;
    for index in 1..adapters.len() {
        if let 1 = adapters[index] - adapters[index-1] {
            one_difference_count += 1;
        } else {
            three_difference_count += 1;
        }
    }
    (one_difference_count, three_difference_count)
}

<<<<<<< HEAD
pub fn find_number_of_arrangements(adapters: &Vec<usize>) -> usize {
    let mut branch_node_to_posssible_immediate_nodes = HashMap::<usize, HashSet<usize>>::new();
    let mut branch_node_to_number_of_arrangements_to_end = HashMap::<usize, usize>::new();
    let mut branch_nodes_descending = Vec::<usize>::new();
    for node in adapters {
        let mut possible_next_nodes = HashSet::<usize>::new();
        if adapters.contains(&(node + 1)) { possible_next_nodes.insert(node + 1); }
        if adapters.contains(&(node + 2)) { possible_next_nodes.insert(node + 2); }
        if adapters.contains(&(node + 3)) { possible_next_nodes.insert(node + 3); }
        if possible_next_nodes.len() > 1 {
            branch_node_to_posssible_immediate_nodes.insert(*node, possible_next_nodes);
            branch_nodes_descending.insert(0, *node);
        }
    }

    let last_node = adapters[adapters.len() - 1];
    branch_nodes_descending.insert(0, last_node);
    branch_node_to_number_of_arrangements_to_end.insert(last_node, 1);
    let mut current_index = 0;
    for branch_node in &branch_nodes_descending {
        if *branch_node == last_node { continue; }
        current_index += 1;
        let mut arrangements_for_parent_node = 0;
        for next_node in &branch_node_to_posssible_immediate_nodes[&branch_node] {
            if let Some(arrangements_for_child_node) = branch_node_to_number_of_arrangements_to_end.get(&next_node) {
                arrangements_for_parent_node += arrangements_for_child_node;
            } else {
                let mut next_branch_index = current_index;
                while branch_nodes_descending[next_branch_index] < *next_node { next_branch_index -= 1; }
                let next_branch = branch_nodes_descending[next_branch_index];
                arrangements_for_parent_node += branch_node_to_number_of_arrangements_to_end[&next_branch];
            }
        }
        branch_node_to_number_of_arrangements_to_end.insert(*branch_node, arrangements_for_parent_node);
    }
    branch_node_to_number_of_arrangements_to_end[&branch_nodes_descending.pop().unwrap()]
}

=======
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn correctly_create_list_of_adapters() {
<<<<<<< HEAD
        let adapters = create_ascending_adapters("test_data/test1.txt");
        assert_eq!(13, adapters.len());
        assert_eq!(0, adapters[0]);
        assert_eq!(22, adapters[12]);
=======
        let adapters = create_adapters("test_data/test1.txt");
        assert_eq!(11, adapters.len());
        assert_eq!(16, adapters[0]);
        assert_eq!(4, adapters[10]);
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
    }

    #[test]
    fn correctly_find_differences_of_1_and_3_jolts() {
<<<<<<< HEAD
        let adapters = create_ascending_adapters("test_data/test1.txt");
        let result = find_differences_of_1_and_3_jolts_from(&adapters);
=======
        let adapters = create_adapters("test_data/test1.txt");
        let result = find_differences_of_1_and_3_jolts_from(adapters);
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
        assert_eq!((7, 5), result);
    }

    #[test]
    fn correctly_find_differences_of_1_and_3_jolts_on_more_complicated_example() {
<<<<<<< HEAD
        let adapters = create_ascending_adapters("test_data/test2.txt");
        let result = find_differences_of_1_and_3_jolts_from(&adapters);
        assert_eq!((22, 10), result);
    }

    #[test]
    fn correctly_find_number_of_arrangements() {
        let adapters = create_ascending_adapters("test_data/test1.txt");
        let arrangements = find_number_of_arrangements(&adapters);
        assert_eq!(8, arrangements);
    }

    #[test]
    fn correctly_find_number_of_arrangements_for_complicated_example() {
        let adapters = create_ascending_adapters("test_data/test2.txt");
        let arrangements = find_number_of_arrangements(&adapters);
        assert_eq!(19208, arrangements);
    }
}

=======
        let adapters = create_adapters("test_data/test2.txt");
        let result = find_differences_of_1_and_3_jolts_from(adapters);
        assert_eq!((22, 10), result);
    }

}
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
