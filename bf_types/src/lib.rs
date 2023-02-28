use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum RawInstruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
    EndLoop,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    row: usize,
    col: usize,
    raw_instruction: RawInstruction,
}

#[derive(Debug)]
pub struct Program {
    file_path: String,
    instructions: Vec<Instruction>,
}

impl RawInstruction {
    /// Convert a char to BF instructions.
    fn from_char(c: char) -> Option<RawInstruction> {
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
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn raw_instruction(&self) -> RawInstruction {
        self.raw_instruction
    }
}
impl Program {
    pub fn new(
        file_path: &str,
        lines: Lines<BufReader<File>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for (row, line) in lines.enumerate() {
            for (col, char) in line?.chars().enumerate() {
                if let Some(raw_instruction) = RawInstruction::from_char(char) {
                    let instruction = Instruction {
                        row: row + 1,
                        col: col + 1,
                        raw_instruction,
                    };
                    instructions.push(instruction);
                }
            }
        }
        Ok(Self {
            file_path: file_path.to_owned(),
            instructions,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = path.as_ref().to_str().ok_or("Converting to &str failed")?;
        let file = File::open(file_path)?;
        let lines = BufReader::new(file).lines();
        Program::new(file_path, lines)
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

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
