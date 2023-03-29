//! A representation of Brainfuck virtual machine and the actual interpret functions.

pub mod brainfuck_runtime_error;
pub mod cell_kind;

use bf_types::{Program, RawInstruction};
use brainfuck_runtime_error::BrainfuckRuntimeError;
use cell_kind::CellKind;
use std::io::{Read, Write};
use std::num::NonZeroUsize;

/// The Brainfuck virtual machine. It can hold data of type T which implements [CellKind] trait.
#[derive(Debug)]
pub struct VM<'a, T>
where
    T: CellKind,
{
    memory: Vec<T>,
    pointer: usize,
    can_extend: bool,
    program_counter: usize,
    program: &'a Program,
}

impl<'a, T> VM<'a, T>
where
    T: CellKind,
{
    /// Constructs a new Brainfuck [VM] and initialize it with a borrow of [Program].
    ///
    /// `memory_size` specifies how much cells the [VM] memory can hold.
    ///
    /// `can_extend` specifies if the memory can extend when it's full.
    ///
    /// `program` is a borrow to a [Program] struct which this [VM] will later interpret.
    /// # Example
    /// ```no_run
    /// # use bf_interp::*;
    /// use std::num::NonZeroUsize;
    /// use bf_types::Program;
    /// # use std::io;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let program = Program::from_file("hello_world.bf")?;
    /// let virtual_machine:VM<u8> = VM::new(NonZeroUsize::new(100).unwrap(), true, &program);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(memory_size: NonZeroUsize, can_extend: bool, program: &'a Program) -> VM<'a, T> {
        let mut memory: Vec<T> = vec![];
        memory.resize(memory_size.get(), T::zero());
        Self {
            memory,
            pointer: 0,
            can_extend,
            program_counter: 0,
            program,
        }
    }

    /// Interpret the borrowed [Program] instance. User has to specify where the input and output will be.
    /// # Example
    /// ```no_run
    /// use bf_types::*;
    /// use bf_interp::*;
    /// use std::io::{stdin, stdout};
    /// use std::num::NonZeroUsize;
    /// # use std::io;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let program = Program::from_file("./hello_world.bf")?;
    /// let memory_size = NonZeroUsize::new(30000).unwrap();
    /// let mut virtual_machine: VM<u8> = VM::new(memory_size, true, &program);
    /// virtual_machine.interpret(&mut stdin(), &mut stdout())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn interpret<R: Read, W: Write>(
        &mut self,
        read_source: &mut R,
        write_destination: &mut W,
    ) -> Result<(), BrainfuckRuntimeError> {
        for (i, instruction) in self.program.instructions().iter().enumerate() {
            self.program_counter = i;
            match instruction.raw_instruction() {
                RawInstruction::MoveLeft => self.move_left()?,
                RawInstruction::MoveRight => self.move_right()?,
                RawInstruction::Increment => self.increment(),
                RawInstruction::Decrement => self.decrement(),
                RawInstruction::Input => self.read_value(read_source)?,
                RawInstruction::Output => self.write_value(write_destination)?,
                _ => println!("{instruction:?}"),
            }
        }
        Ok(())
    }

    /// Move [VM] pointer one place to the left. Will cause a [BrainfuckRuntimeError] if the pointer is already at
    /// position 0.
    fn move_left(&mut self) -> Result<(), BrainfuckRuntimeError> {
        if self.pointer == 0 {
            return Err(BrainfuckRuntimeError::CannotMoveLeftError(
                self.program.file_path().to_owned(),
                *self
                    .program
                    .instructions()
                    .get(self.program_counter)
                    .unwrap(),
            ));
        }
        self.pointer -= 1;
        Ok(())
    }

    /// Move [VM] pointer one place to the right. If the pointer is already at the right most position, then this method
    /// will either double [VM]'s memory size if it's allowed, or return a [BrainfuckRuntimeError] saying invalid
    /// operation.
    fn move_right(&mut self) -> Result<(), BrainfuckRuntimeError> {
        let memory_size = self.memory.len();

        if self.pointer == memory_size - 1 && !self.can_extend {
            return Err(BrainfuckRuntimeError::CannotMoveRightError(
                self.program.file_path().to_owned(),
                *self
                    .program
                    .instructions()
                    .get(self.program_counter)
                    .unwrap(),
            ));
        } else if self.pointer == memory_size - 1 {
            self.memory.resize(2 * memory_size, T::zero());
        }

        self.pointer += 1;
        Ok(())
    }

    /// Increment the value pointed by [VM] pointer.
    fn increment(&mut self) {
        self.memory[self.pointer].increment();
    }

    /// Decrement the value pointed by [VM] pointer.
    fn decrement(&mut self) {
        self.memory[self.pointer].decrement();
    }

    /// Read a u8 value from user specified reading source.
    fn read_value<R: Read>(&mut self, input_source: &mut R) -> Result<(), BrainfuckRuntimeError> {
        let mut buf = [0; 1];
        let result = input_source.read_exact(&mut buf);

        if let Err(e) = result {
            return Err(BrainfuckRuntimeError::CannotReadInputError(
                e,
                self.program.file_path().to_owned(),
                *self
                    .program
                    .instructions()
                    .get(self.program_counter)
                    .unwrap(),
            ));
        }

        self.memory[self.pointer].set_value(buf[0]);

        Ok(())
    }

    /// Write a cell value as ASCII to user specified write destination.
    fn write_value<W: Write>(
        &self,
        write_destination: &mut W,
    ) -> Result<(), BrainfuckRuntimeError> {
        let value = self.memory()[self.pointer].get_value();
        let result = write_destination.write_all(&[value]);

        if let Err(e) = result {
            return Err(BrainfuckRuntimeError::CannotWriteOutputError(
                e,
                self.program.file_path().to_owned(),
                *self
                    .program
                    .instructions()
                    .get(self.program_counter)
                    .unwrap(),
            ));
        }
        Ok(())
    }

    /// Getter.
    pub fn memory(&self) -> &[T] {
        &self.memory
    }

    /// Getter.
    pub fn can_extend(&self) -> bool {
        self.can_extend
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bf_types::Instruction;
    use std::io::{stdin, stdout, Cursor};

    /// Should create a VM with specified number of cells in memory.
    #[test]
    fn specified_memory_size() {
        let program = Program::new("", "");
        let virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), true, &program);
        assert_eq!(virtual_machine.memory().len(), 10);
    }

    /// Should initialize pointer location to 0.
    #[test]
    fn initialize_pointer_location() {
        let program = Program::new("", "");
        let virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), true, &program);
        assert_eq!(virtual_machine.pointer, 0);
    }

    /// Should successfully move pointer one place to the left.
    #[test]
    fn successfully_move_pointer_left() {
        let program = Program::new("", "<");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), false, &program);
        virtual_machine.pointer = 1;
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());
        assert!(result.is_ok());
        assert_eq!(virtual_machine.pointer, 0);
    }

    /// Should failed moving pointer one place to the left.
    #[test]
    fn unsuccessfully_move_pointer_left() {
        let program = Program::new("", "<");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), false, &program);
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());

        match result {
            Err(BrainfuckRuntimeError::CannotMoveLeftError(file_path, ins)) => {
                assert_eq!(file_path.to_str().unwrap(), "");
                assert_eq!(ins, Instruction::new(1, 1, RawInstruction::MoveLeft));
            }
            _ => panic!("Unrecognized error type"),
        }

        assert_eq!(virtual_machine.pointer, 0);
    }

    /// Should successfully move pointer one place to the right in normal case.
    #[test]
    fn successfully_move_pointer_right_normal_case() {
        let program = Program::new("", ">");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), false, &program);
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());
        assert!(result.is_ok());
        assert_eq!(virtual_machine.pointer, 1);
    }

    /// Should successfully move pointer one place to the right when extension is allowed and needed.
    #[test]
    fn successfully_move_pointer_right_at_right_edge() {
        let program = Program::new("", ">");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), true, &program);
        virtual_machine.pointer = 1;
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());
        assert!(result.is_ok());
        assert_eq!(virtual_machine.pointer, 2);
        assert_eq!(virtual_machine.memory.len(), 4);
    }

    /// Should failed moving pointer one place to the right when at the right edge and cannot extend.
    #[test]
    fn unsuccessfully_move_pointer_right() {
        let program = Program::new("", ">");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), false, &program);
        virtual_machine.pointer = 1;
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());

        match result {
            Err(BrainfuckRuntimeError::CannotMoveRightError(file_path, ins)) => {
                assert_eq!(file_path.to_str().unwrap(), "");
                assert_eq!(ins, Instruction::new(1, 1, RawInstruction::MoveRight));
            }
            _ => panic!("Unrecognized error type"),
        }

        assert_eq!(virtual_machine.pointer, 1);
    }

    /// Should increment cell value by one.
    #[test]
    fn increase_by_one() {
        let program = Program::new("", "+");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), false, &program);
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());

        assert!(result.is_ok());
        assert_eq!(virtual_machine.memory()[0], 1_u8);
    }

    /// Should decrement cell value by one.
    #[test]
    fn decrease_by_one() {
        let program = Program::new("", "-");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), false, &program);
        virtual_machine.memory[0] = 1;
        let result = virtual_machine.interpret(&mut stdin(), &mut stdout());

        assert!(result.is_ok());
        assert_eq!(virtual_machine.memory()[0], 0_u8);
    }

    /// Should successfully set a memory cell to a u8 value.
    #[test]
    fn successfully_set_memory_cell() {
        let program = Program::new("", ",");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), false, &program);
        let mut read_source = Cursor::new(vec![65]);
        let result = virtual_machine.interpret(&mut read_source, &mut stdout());

        assert!(result.is_ok());
        assert_eq!(virtual_machine.memory()[0], 65);
    }

    /// Should fail due to Unexpected EOF error.
    #[test]
    fn set_memory_cell_failed_due_to_eof() {
        let program = Program::new("", ",");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), false, &program);
        let mut read_source = Cursor::new(vec![]);
        let result = virtual_machine.interpret(&mut read_source, &mut stdout());

        match result {
            Err(BrainfuckRuntimeError::CannotReadInputError(io_err, file_path, ins)) => {
                assert_eq!(io_err.kind(), std::io::ErrorKind::UnexpectedEof);
                assert_eq!(file_path.to_str().unwrap(), "");
                assert_eq!(ins, Instruction::new(1, 1, RawInstruction::Input));
            }
            _ => panic!("Unrecognized error type"),
        }

        assert_eq!(virtual_machine.memory()[0], 0);
    }

    /// Should successfully write a memory cell content to write destination.
    #[test]
    fn successfully_write_memory_cell_to_destination() {
        let program = Program::new("", ".");
        let mut virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(2).unwrap(), false, &program);
        virtual_machine.memory[0] = 65;
        let mut write_destination = Cursor::new(vec![]);
        let result = virtual_machine.interpret(&mut stdin(), &mut write_destination);

        assert!(result.is_ok());
        assert_eq!(write_destination.into_inner(), vec![65]);
    }
}
