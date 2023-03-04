//! This library converts text brainfuck code into Rust-understandable format.

use std::fmt;
use std::fs::read_to_string;
use std::path::Path;

/// An enum type to represent the 8 Brainfuck instructions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RawInstruction {
    /// Move VM pointer to left.
    MoveLeft,

    /// Move VM pointer to right.
    MoveRight,

    /// Increment the value pointed by VM pointer by one.
    Increment,

    /// Decrement the value pointed by VM pointer by one.
    Decrement,

    /// Take value from user input.
    Input,

    /// Output the value pointed by VM pointer as ASCII
    Output,

    /// Loop starts here.
    BeginLoop,

    /// Loop ends here.
    EndLoop,
}

/// A structure to represent a brainfuck instruction, an instruction consists of its row and col number in source file,
/// and the instruction type which is defined by enum RawInstruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    row: usize,
    col: usize,
    raw_instruction: RawInstruction,
}

/// A structure to represent a brainfuck program. A program consists of its source code file name, and a vector of its
/// instructions.
#[derive(Debug)]
pub struct Program {
    file_path: String,
    instructions: Vec<Instruction>,
}

impl RawInstruction {
    /// Convert a char to BF instructions. Return type is Option as all brainfuck comment will be converted into None.
    /// # Example
    /// ```
    /// # use bf_types::RawInstruction;
    /// let increment: char = '+';
    /// let parsed = RawInstruction::from_char(increment);
    /// assert_eq!(parsed, Some(RawInstruction::Increment));
    /// 
    /// let comment: char = 'e';
    /// let parsed = RawInstruction::from_char(comment);
    /// assert_eq!(parsed, None);
    /// ```
    pub fn from_char(c: char) -> Option<RawInstruction> {
        match c {
            '<' => Some(RawInstruction::MoveLeft),
            '>' => Some(RawInstruction::MoveRight),
            '+' => Some(RawInstruction::Increment),
            '-' => Some(RawInstruction::Decrement),
            '.' => Some(RawInstruction::Output),
            ',' => Some(RawInstruction::Input),
            '[' => Some(RawInstruction::BeginLoop),
            ']' => Some(RawInstruction::EndLoop),
            _ => None,
        }
    }
}

impl fmt::Display for RawInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                RawInstruction::MoveLeft => "Move pointer to left",
                RawInstruction::MoveRight => "Move pointer to right",
                RawInstruction::Increment => "Increment current location",
                RawInstruction::Decrement => "Decrement current location",
                RawInstruction::Output => "Output current location as ASCII",
                RawInstruction::Input => "Input ASCII to current location",
                RawInstruction::BeginLoop => "Start looping",
                RawInstruction::EndLoop => "End looping",
            }
        )
    }
}

impl Instruction {
    /// Build a new instance of Instruction.
    /// # Example
    /// ```
    /// # use bf_types::*;
    /// let raw_instruction = RawInstruction::from_char('+').unwrap();
    /// let instruction = Instruction::new(1, 1, raw_instruction);
    /// ```
    pub fn new(row: usize, col: usize, raw_instruction: RawInstruction) -> Self {
        Self {
            row,
            col,
            raw_instruction,
        }
    }

    /// Getter for instruction row number.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Getter for instruction col number.
    pub fn col(&self) -> usize {
        self.col
    }

    /// Getter for instruction enum type.
    pub fn raw_instruction(&self) -> RawInstruction {
        self.raw_instruction
    }
}

impl Program {
    /// Creates a brainfuck Program with a file name in a path-like format and its content in a string-ike format.
    fn new<P: AsRef<Path>>(file_path: P, lines: &str) -> Self {
        let mut instructions: Vec<Instruction> = Vec::new();
        let lines = lines.split('\n');
        for (row, line) in lines.enumerate() {
            for (col, char) in line.chars().enumerate() {
                if let Some(raw_instruction) = RawInstruction::from_char(char) {
                    instructions.push(Instruction::new(row + 1, col + 1, raw_instruction));
                }
            }
        }
        Self {
            file_path: file_path
                .as_ref()
                .to_string_lossy()
                .to_string(),
            instructions,
        }
    }

    /// Creates a brainfuck Program from a file, which is specified as a path-like.
    /// # Example
    /// ```no_run
    /// # use bf_types;
    /// let file_path = "./hello_world.bf";
    /// let program = bf_types::Program::from_file(file_path);
    /// ```
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let binding = read_to_string(&file_path)?;
        let lines = binding.as_str();
        Ok(Program::new(file_path, lines))
    }

    /// Getter for file path.
    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    /// Getter for the list of instructions.
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions.as_slice()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut representation: Vec<String> = Vec::new();
        representation.push(format!("File: {}", self.file_path()));
        for &instruction in self.instructions() {
            representation.push(format!(
                "Row: {}, Col: {}: {}",
                instruction.row(),
                instruction.col(),
                instruction.raw_instruction()
            ));
        }
        write!(f, "{}", representation.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    /// Test for correctly parse brainfuck code into corresponding enum type.
    #[test]
    fn parse_instructions() -> Result<(), String> {
        let bf_code = "<>+some none bf comments!85\t\n()&\\-[],.";
        let expected = vec![
            RawInstruction::MoveLeft,
            RawInstruction::MoveRight,
            RawInstruction::Increment,
            RawInstruction::Decrement,
            RawInstruction::BeginLoop,
            RawInstruction::EndLoop,
        ];
        let parsed_program = Program::new("file_path", bf_code);
            for (actual_instruction, expected_instruction) in parsed_program
                .instructions()
                .into_iter()
                .zip(expected.into_iter())
            {
                if actual_instruction.raw_instruction() != expected_instruction {
                    return Err(format!(
                        "Expects {:?}, but found {:?}",
                        expected_instruction,
                        actual_instruction.raw_instruction(),
                    ));
                }
            }
                Ok(())
    }

    #[test]
    /// Test for correctly parse row and col location of each brainfuck instructions.
    fn parse_locations() -> Result<(), String> {
        let bf_code = indoc!("
            <>
            some comment
            even comment in another language
            中文+-  
        ");
        let parsed_program = Program::new("file_path", bf_code);
        let expected = vec![
            Instruction{row: 1, col: 1, raw_instruction: RawInstruction::MoveLeft},
            Instruction{row: 1, col: 2, raw_instruction: RawInstruction::MoveRight},
            Instruction{row: 4, col: 3, raw_instruction: RawInstruction::Increment},
            Instruction{row: 4, col: 4, raw_instruction: RawInstruction::Decrement},
        ];
        for (actual_instruction, expected_instruction) in parsed_program.instructions().into_iter().zip(expected.into_iter()) {
            if *actual_instruction != expected_instruction {
                return Err(String::from(format!(
                    "Expects {:?}, found {:?}",
                    expected_instruction,
                    *actual_instruction
                )))
            }
        }
        Ok(())
    }
}
