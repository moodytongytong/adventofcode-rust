use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub struct GameConsole {
    instructions: Vec<String>,
    accumulator: isize,
    position: usize,
}

impl GameConsole {
    pub fn new(filepath: &str) -> Self {
        let mut instructions = Vec::<String>::new();
        if let Ok(lines) = read_lines(filepath) {
            for line in lines {
                if let Ok(instruction) = line {
                    instructions.push(instruction);
                }
            }
        }
        Self { 
            instructions,
            accumulator: 0,
            position: 0,
        }
    }

    fn acc(&mut self, index: usize) {
        let change: isize = self.instructions[index][4..].parse().unwrap();
        self.accumulator += change;
        self.position += 1;
    }

    fn jmp(&mut self, index: usize) {
        let change: isize = self.instructions[index][4..].parse().unwrap();
        self.position = (change + index as isize) as usize;
    }

    fn nop(&mut self, _index: usize) {
        self.position += 1;
    }

    fn operate(&mut self, index: usize) {
        let operation = &self.instructions[index][0..3];
        match operation {
            "acc" => self.acc(index),
            "jmp" => self.jmp(index),
            _ => self.nop(index),
        }
    }

    fn next(&mut self) {
        self.operate(self.position);
    }

    pub fn find_accumulator_before_loop(&mut self) -> isize {
        let mut visited_indices = HashSet::<usize>::new();
        while !visited_indices.contains(&self.position) {
            visited_indices.insert(self.position);
            self.next();
        }
        self.accumulator
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions_correctly_read_in() {
        let console = GameConsole::new("test_data/test1.txt");
        assert_eq!(9, console.instructions.len());
    }

    #[test]
    fn test_acc_works_correctly() {
        let mut console = GameConsole::new("test_data/test1.txt");
        console.acc(1);
        assert_eq!(1, console.accumulator);
    }

    #[test]
    fn test_jmp_works_correctly() {
        let mut console = GameConsole::new("test_data/test1.txt");
        console.jmp(2);
        assert_eq!(6, console.position);
    }

    #[test]
    fn correctly_identify_operation() {
        let mut console = GameConsole::new("test_data/test1.txt");
        console.operate(1);
        assert_eq!(1, console.accumulator);
        console.operate(2);
        assert_eq!(6, console.position);
    }

    #[test]
    fn instructions_increments_correctly() {
        let mut console = GameConsole::new("test_data/test1.txt");
        console.next();
        assert_eq!(0, console.accumulator);
        assert_eq!(1, console.position);
        console.next();
        assert_eq!(1, console.accumulator);
        assert_eq!(2, console.position);
        console.next();
        assert_eq!(1, console.accumulator);
        assert_eq!(6, console.position);
    }

    #[test]
    fn correctly_finds_accumulator_before_looping_back() {
        let mut console = GameConsole::new("test_data/test1.txt");
        assert_eq!(5, console.find_accumulator_before_loop());
    }
}