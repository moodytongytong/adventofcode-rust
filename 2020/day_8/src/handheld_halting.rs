use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub struct GameConsole {
    instructions: Vec<String>,
    accumulator: isize,
    position: usize,
    filepath: String,
}

impl GameConsole {
    pub fn new(filepath: &str) -> Self {
        let mut instructions = Vec::<String>::new();
        load_instructions_from(filepath, &mut instructions);
        Self { 
            instructions,
            accumulator: 0,
            position: 0,
            filepath: filepath.to_string()
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

    pub fn accumulator_value_when_terminating_normally(&mut self) -> isize {
        let wrong_instruction_index = self.find_index_of_wrong_instruction();
        self.reset();
        self.update_instruction(wrong_instruction_index);
        while self.position < self.instructions.len() {
            self.next();
        }
        self.accumulator
    }

    fn is_loop(&mut self) -> bool {
        let mut visited_indices = HashSet::<usize>::new();
        while !visited_indices.contains(&self.position) {
            visited_indices.insert(self.position);
            self.next();
            if self.position == self.instructions.len() {
                return false
            }
        }
        true
    }

    fn update_instruction(&mut self, index: usize) {
        let operation = &self.instructions[index][0..3];
        let operand = &self.instructions[index][3..];
        match operation {
            "nop" => self.instructions[index] = "jmp".to_owned() + operand,
            "jmp" => self.instructions[index] = "nop".to_owned() + operand,
            _ => {},
        }
    }

    fn reset(&mut self) {
        load_instructions_from(&self.filepath, &mut self.instructions);
        self.accumulator = 0;
        self.position = 0;
    }

    fn find_index_of_wrong_instruction(&mut self) -> usize {
        for index in 0..self.instructions.len() {
            self.reset();
            self.update_instruction(index);
            if let false = self.is_loop() {
                return index;
            }
        }
        0
    }
}

fn load_instructions_from(filepath: &str, instructions : &mut Vec<String>) {
    instructions.clear();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(instruction) = line {
                instructions.push(instruction);
            }
        }
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

    #[test]
    fn find_correct_accumulator_value_when_terminating_normally() {
        let mut console = GameConsole::new("test_data/test1.txt");
        assert_eq!(8, console.accumulator_value_when_terminating_normally());
    }

    #[test]
    fn determine_if_instructions_are_a_loop() {
        let mut console = GameConsole::new("test_data/test1.txt");
        assert!(console.is_loop());
        console.update_instruction(7);
        assert_eq!(false, console.is_loop());
    }

    #[test]
    fn new_console_created_from_new_instruction() {
        let mut console = GameConsole::new("test_data/test1.txt");
        console.update_instruction(0);
        assert_eq!("jmp +0", console.instructions[0]);
        console.update_instruction(2);
        assert_eq!("nop +4", console.instructions[2]);
    }

    #[test]
    fn console_can_be_reset() {
        let mut console = GameConsole::new("test_data/test1.txt");
        console.find_accumulator_before_loop();
        console.update_instruction(0);
        console.reset();
        assert_eq!(0, console.accumulator);
        assert_eq!(0, console.position);
        assert_eq!("nop +0", console.instructions[0]);
    }

    #[test]
    fn find_index_of_the_wrong_instruction() {
        let mut console = GameConsole::new("test_data/test1.txt");
        assert_eq!(7, console.find_index_of_wrong_instruction());
    }
}