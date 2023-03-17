//! A representation of Brainfuck virtual machine and the actual interpret functions.

use bf_types::Program;
use num_traits::{Bounded, Num};
use std::cmp::PartialOrd;
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::ops::{AddAssign, SubAssign};

/// The Brainfuck virtual machine. It can hold data of type T, but T must be a number-like type such as [u8], [i32].
#[derive(Debug)]
pub struct VM<T>
where
    T: Num + Bounded + AddAssign + SubAssign + Copy + PartialOrd,
{
    memory: Vec<T>,
    pointer: usize,
    can_extend: bool,
}

/// Brainfuck specific errors we might encounter.
#[derive(Debug)]
pub enum BrainfuckError {
    CannotMoveLeftError,
    CannotMoveRightError,
    CannotIncrementError,
    CannotDecrementError,
    InvalidValueError,
}

impl fmt::Display for BrainfuckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrainfuckError::CannotMoveLeftError => write!(f, "Pointer at position 0."),
            BrainfuckError::CannotMoveRightError => write!(
                f,
                "Pointer at right edge and cannot extend virtual machine memory."
            ),
            BrainfuckError::CannotIncrementError => write!(f, "Current value is max of type T."),
            BrainfuckError::CannotDecrementError => write!(f, "Current value is min of type T."),
            BrainfuckError::InvalidValueError => write!(f, "Input value is outside of range T."),
        }
    }
}

impl Error for BrainfuckError {}

impl<T> VM<T>
where
    T: Num + Bounded + AddAssign + SubAssign + Copy + PartialOrd,
{
    /// Constructs a new Brainfuck [VM] and initialize it.
    ///
    /// `memory_size` specifies how much cells the [VM] memory can hold, and it will use the default value `30,000` if
    /// `None` is provided.
    ///
    /// `can_extend` specifies if the memory can extend when it's full.
    /// # Example
    /// ```
    /// # use bf_interp::*;
    /// use std::num::NonZeroUsize;
    /// let virtual_machine:VM<u8> = VM::new(NonZeroUsize::new(100).unwrap(), true);
    /// ```
    pub fn new(memory_size: NonZeroUsize, can_extend: bool) -> VM<T> {
        let mut memory: Vec<T> = vec![];
        memory.resize(memory_size.get(), T::zero());
        Self {
            memory,
            pointer: 0,
            can_extend,
        }
    }

    /// Interpret a given Brainfuck [Program].
    /// # Example
    /// ```no_run
    /// use bf_types::*;
    /// use bf_interp::*;
    /// use std::num::NonZeroUsize;
    /// # use std::io;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let program = Program::from_file("./hello_world.bf")?;
    /// let memory_size = NonZeroUsize::new(30000).unwrap();
    /// let virtual_machine: VM<u8> = VM::new(memory_size, true);
    /// virtual_machine.interpret(&program);
    /// # Ok(())
    /// # }
    /// ```
    pub fn interpret(&self, program: &Program) {
        println!("{program}");
    }

    // Brainfuck instructions (other than loop).
    // fn move_left(&mut self) -> Result<(), BrainfuckError> {
    //     if self.pointer == 0 {
    //         return Err(BrainfuckError::CannotMoveLeftError);
    //     }
    //     self.pointer -= 1;
    //     Ok(())
    // }

    // fn move_right(&mut self) -> Result<(), BrainfuckError> {
    //     if self.pointer == self.memory.capacity() - 1 && !self.can_extend {
    //         return Err(BrainfuckError::CannotMoveRightError);
    //     } else if self.pointer == self.memory.capacity() - 1 {
    //         self.memory.reserve_exact(self.memory.len());
    //         self.memory.push(T::zero());
    //     } else if self.pointer == self.memory.len() - 1 {
    //         self.memory.push(T::zero());
    //     }
    //     self.pointer += 1;
    //     Ok(())
    // }

    // fn increment(&mut self) -> Result<(), BrainfuckError> {
    //     if self.memory[self.pointer] == T::max_value() {
    //         return Err(BrainfuckError::CannotIncrementError);
    //     }
    //     self.memory[self.pointer] += T::one();
    //     Ok(())
    // }

    // fn decrement(&mut self) -> Result<(), BrainfuckError> {
    //     if self.memory[self.pointer] == T::min_value() {
    //         return Err(BrainfuckError::CannotDecrementError);
    //     }
    //     self.memory[self.pointer] -= T::one();
    //     Ok(())
    // }

    // fn output(&self) -> Result<T, BrainfuckError> {
    //     Ok(self.memory[self.pointer])
    // }

    // fn input(&mut self, value: T) -> Result<(), BrainfuckError> {
    //     if T::min_value() <= value && value <= T::max_value() {
    //         self.memory[self.pointer] = value;
    //         return Ok(());
    //     }
    //     Err(BrainfuckError::InvalidValueError)
    // }

    /// Getter.
    pub fn memory(&self) -> &[T] {
        &self.memory
    }

    /// Getter.
    pub fn pointer(&self) -> usize {
        self.pointer
    }

    /// Getter.
    pub fn can_extend(&self) -> bool {
        self.can_extend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Should create a VM with specified number of cells in memory.
    #[test]
    fn specified_memory_size() {
        let virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), true);
        assert_eq!(virtual_machine.memory().len(), 10);
    }

    /// Should initialize pointer location to 0.
    #[test]
    fn initialize_pointer_location() {
        let virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10).unwrap(), true);
        assert_eq!(virtual_machine.pointer(), 0);
    }
}
