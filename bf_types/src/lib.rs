//! Converts text brainfuck code into Rust-understandable format.

use std::error::Error;
use std::fmt;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/// A representation of the 8 Brainfuck instructions.
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

/// A representation of a brainfuck instruction, an instruction consists of its row and col number in source file,
/// and the instruction type which is defined by [RawInstruction].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    row: usize,
    col: usize,
    raw_instruction: RawInstruction,
}

/// A representation of a brainfuck program.
#[derive(Debug)]
pub struct Program {
    file_path: PathBuf,
    instructions: Vec<Instruction>,
}

/// A representation for errors caused by incompatible brackets in Brainfuck source code.
#[derive(Debug)]
pub enum IncompatibleBracket {
    /// A close bracket has no corresponding open bracket.
    MissingOpenBracket {
        file_path: PathBuf,
        close_bracket: Instruction,
    },

    /// An open bracket has no corresponding close bracket.
    MissingCloseBracket {
        file_path: PathBuf,
        open_bracket: Instruction,
    },
}

impl RawInstruction {
    /// Convert a char value to BF [RawInstruction]. All brainfuck comment will be converted into None.
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
    /// Build a new instance of [Instruction].
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

    /// Getter.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Getter.
    pub fn col(&self) -> usize {
        self.col
    }

    /// Getter.
    pub fn raw_instruction(&self) -> RawInstruction {
        self.raw_instruction
    }
}

impl Program {
    /// Creates a Brainfuck [Program] with a file name in a path-like format and its content in a string-like format.
    pub fn new<P: AsRef<Path>>(file_path: P, lines: &str) -> Self {
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
            file_path: file_path.as_ref().to_owned(),
            instructions,
        }
    }

    /// Creates a Brainfuck [Program] from a file, which is specified as a path-like.
    /// # Example
    /// ```no_run
    /// # use bf_types::*;
    /// let file_path = "./hello_world.bf";
    /// let program = Program::from_file(file_path);
    /// ```
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, std::io::Error> {
        let binding = read_to_string(&file_path)?;
        let lines = binding.as_str();
        Ok(Program::new(file_path, lines))
    }

    /// Check if a piece of Brainfuck [Program] is valid. I.e., if it has a matching brackets.
    /// # Example
    /// ```no_run
    /// use bf_types::*;
    /// # use std::io;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let program = Program::from_file("./hello_world.bf")?;
    /// program.validate()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn validate(&self) -> Result<(), IncompatibleBracket> {
        let mut stack: Vec<&Instruction> = Vec::with_capacity(self.instructions().len());
        for (_idx, ins) in self.instructions().iter().enumerate() {
            if ins.raw_instruction() == RawInstruction::BeginLoop {
                stack.push(ins);
            } else if ins.raw_instruction() == RawInstruction::EndLoop {
                if stack.is_empty() {
                    return Err(IncompatibleBracket::MissingOpenBracket {
                        file_path: self.file_path().to_owned(),
                        close_bracket: *ins,
                    });
                }
                stack.pop();
            }
        }
        if let Some(ins) = stack.first() {
            return Err(IncompatibleBracket::MissingCloseBracket {
                file_path: self.file_path().to_owned(),
                open_bracket: **ins,
            });
        }
        Ok(())
    }

    /// Getter.
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    /// Getter.
    pub fn instructions(&self) -> &[Instruction] {
        self.instructions.as_slice()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ins in self.instructions() {
            writeln!(
                f,
                "[{}:{}:{}] {}",
                self.file_path().display(),
                ins.row(),
                ins.col(),
                ins.raw_instruction()
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for IncompatibleBracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncompatibleBracket::MissingOpenBracket {
                file_path,
                close_bracket,
            } => {
                write!(
                    f,
                    "Found ']' at [{}:{}:{}] but no matching '[' found",
                    file_path.display(),
                    close_bracket.row(),
                    close_bracket.col()
                )
            }
            IncompatibleBracket::MissingCloseBracket {
                file_path,
                open_bracket,
            } => {
                write!(
                    f,
                    "Found '[' at [{}:{}:{}] but no matching ']' found",
                    file_path.display(),
                    open_bracket.row(),
                    open_bracket.col()
                )
            }
        }
    }
}

impl Error for IncompatibleBracket {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    /// Should correctly parse brainfuck code into corresponding [RawInstruction].
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
            .iter()
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
    /// Should correctly parse row and col location of each brainfuck [Instruction]s.
    fn parse_locations() -> Result<(), String> {
        let bf_code = indoc!(
            "
            <>
            some comment
            even comment in another language
            中文+-  
        "
        );
        let parsed_program = Program::new("file_path", bf_code);
        let expected = vec![
            Instruction {
                row: 1,
                col: 1,
                raw_instruction: RawInstruction::MoveLeft,
            },
            Instruction {
                row: 1,
                col: 2,
                raw_instruction: RawInstruction::MoveRight,
            },
            Instruction {
                row: 4,
                col: 3,
                raw_instruction: RawInstruction::Increment,
            },
            Instruction {
                row: 4,
                col: 4,
                raw_instruction: RawInstruction::Decrement,
            },
        ];
        for (actual_instruction, expected_instruction) in parsed_program
            .instructions()
            .iter()
            .zip(expected.into_iter())
        {
            if *actual_instruction != expected_instruction {
                return Err(format!(
                    "Expects {:?}, found {:?}",
                    expected_instruction, *actual_instruction
                ));
            }
        }
        Ok(())
    }

    /// Should identify unclosed open in Brainfuck source code.
    #[test]
    fn should_identify_unclosed_open() {
        let program = Program::new("", "[[[]");
        let result = program.validate();

        assert!(result.is_err());
        match result {
            Err(IncompatibleBracket::MissingCloseBracket {
                file_path,
                open_bracket,
            }) => {
                assert_eq!(file_path.to_str().unwrap(), "");
                assert_eq!(
                    open_bracket,
                    Instruction::new(1, 1, RawInstruction::BeginLoop)
                );
            }
            _ => panic!("Unrecognized error"),
        }
    }

    /// Should identify unopened close in Brainfuck source code.
    #[test]
    fn should_identify_unopened_close() {
        let program = Program::new("", "]");
        let result = program.validate();

        assert!(result.is_err());
        match result {
            Err(IncompatibleBracket::MissingOpenBracket {
                file_path,
                close_bracket,
            }) => {
                assert_eq!(file_path.to_str().unwrap(), "");
                assert_eq!(
                    close_bracket,
                    Instruction::new(1, 1, RawInstruction::EndLoop)
                );
            }
            _ => panic!("Unrecognized error"),
        }
    }

    /// Should identify matching brackets in Brainfuck source code.
    #[test]
    fn should_identify_matching_brackets() {
        let program = Program::new("", "[][[]]");
        let result = program.validate();

        assert!(result.is_ok());
    }
}
