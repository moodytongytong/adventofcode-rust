use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;

struct GameConsole {
    instructions: Vec<String>,
    accumulator: usize,
    position: usize,
}

impl GameConsole {
    fn new(filepath: &str) -> Self {
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
        let change: usize = self.instructions[index][4..].parse().unwrap();
        self.accumulator += change;
    }

    fn jmp(&mut self, index: usize) {
        let change: usize = self.instructions[index][4..].parse().unwrap();
        self.position = change + index;
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
}