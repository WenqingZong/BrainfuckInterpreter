use std::env;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// The 8 BrainFuck instructions.
#[derive(Clone, Copy, Debug)]
enum RawInstruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
    EndLoop,
}

impl RawInstruction {
    /// Convert a char to BF command.
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

/// A structure to hold the instructions and its locations in file.
#[derive(Debug)]
struct Instruction {
    raw: RawInstruction,
    line: usize,
    col: usize,
}

impl Instruction {
    /// Some getters.
    fn raw(&self) -> RawInstruction {
        self.raw
    }

    fn line(&self) -> usize {
        self.line
    }

    fn col(&self) -> usize {
        self.col
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[inputfile:{}:{}] {}",
            self.line(),
            self.col(),
            self.raw(),
        )
    }
}

/// Read BrainFuck source file specified by file_path, and return a vector holding all the valid BF code.
fn read_source_file(file_path: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    let mut ins_vec: Vec<Instruction> = vec![];
    let file = File::open(file_path)?;
    let lines = BufReader::new(file).lines();
    for (row, line) in lines.enumerate() {
        for (col, char) in line?.chars().enumerate() {
            if let Some(raw_instruction) = RawInstruction::from_char(char) {
                ins_vec.push(Instruction {
                    raw: raw_instruction,
                    line: row + 1,
                    col: col + 1,
                });
                if cfg!(debug_assertions) {
                    println!("{:?}", ins_vec.last());
                }
            }
        }
    }
    Ok(ins_vec)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).ok_or("Please specify an input file")?;
    let instructions = read_source_file(&file_path);

    for instruction in instructions? {
        println!("{}", instruction);
    }
    Ok(())
}
