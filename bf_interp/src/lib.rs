//! Library for Brainfuck virtual machine and the actual interpret functions.

use bf_types::Program;
use num_traits::{Bounded, Num};
use std::cmp::PartialOrd;
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::ops::{AddAssign, SubAssign};

/// The Brainfuck virtual machine. The VM can hold data of type T, but T must be a number-like type such as u8, i32.
/// The VM contains a vector of type T to hold memory, and a pointer to indicate current index. Boolean value
/// 'can_extend' indicates if the memory is allowed to extend.
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
    /// Constructs a new Brainfuck virtual machine and initialize it.
    /// If the memory_size if None, then the default value 30000 will be used.
    /// # Example
    /// ```
    /// # use bf_interp::*;
    /// use std::num::NonZeroUsize;
    /// let virtual_machine:VM<u8> = VM::new(NonZeroUsize::new(100), true);
    /// ```
    pub fn new(memory_size: Option<NonZeroUsize>, can_extend: bool) -> VM<T> {
        let memory_size = match memory_size {
            Some(size) => size.get(),
            None => 30000,
        };
        let mut memory: Vec<T> = Vec::with_capacity(memory_size);
        memory.push(T::zero());
        Self {
            memory,
            pointer: 0,
            can_extend,
        }
    }

    /// Interpret a given Brainfuck program.
    /// # Example
    /// ```no_run
    /// use bf_types::*;
    /// use bf_interp::*;
    /// # use std::io;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let program = Program::from_file("./hello_world.bf")?;
    /// let virtual_machine: VM<u8> = VM::new(None, true);
    /// virtual_machine.interpret(&program);
    /// # Ok(())
    /// # }
    /// ```
    pub fn interpret(&self, program: &Program) {
        println!("{}", program);
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

    /// Getter for virtual machine memory slice.
    pub fn memory(&self) -> &[T] {
        &self.memory
    }

    /// Getter for virtual machine pointer.
    pub fn pointer(&self) -> usize {
        self.pointer
    }

    /// Getter for if the virtual can extend its memory.
    pub fn can_extend(&self) -> bool {
        self.can_extend
    }
}
