//! Brainfuck specific errors we might encounter at run time.
use bf_types::Instruction;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum BrainfuckRuntimeError {
    /// The pointer is already at the left most position.
    CannotMoveLeftError(PathBuf, Instruction),

    /// The pointer is already at the right most position and the [VM](crate::VM) cannot extend its memory.
    CannotMoveRightError(PathBuf, Instruction),

    /// IO error for failed to read user input as [u8].
    CannotReadInputError(std::io::Error, PathBuf, Instruction),

    /// IO error for failed to output Brainfuck result as [u8].
    CannotWriteOutputError(std::io::Error, PathBuf, Instruction),
}

impl fmt::Display for BrainfuckRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrainfuckRuntimeError::CannotMoveLeftError(file_path, ins) =>
                write!(
                    f,
                    "Pointer already at 0 but [{}:{}:{}] still wants to move it left",
                    file_path.display(), ins.row(), ins.col()),
            BrainfuckRuntimeError::CannotMoveRightError(file_path, ins) =>
                write!(
                    f,
                    "Pointer already at right edge and VM is not extendable, but [{}:{}:{}] still wants to move it right",
                    file_path.display(), ins.row(), ins.col()
                ),
            BrainfuckRuntimeError::CannotReadInputError(io_error, file_path, ins) =>
                write!(
                    f,
                    "[{}:{}:{}] wants to read a value but failed due to {}",
                    file_path.display(), ins.row(), ins.col(), io_error
                ),
            BrainfuckRuntimeError::CannotWriteOutputError(io_error, file_path, ins) =>
                write!(
                    f,
                    "[{}:{}:{}] wants to write a value but failed due to {}",
                    file_path.display(), ins.row(), ins.col(), io_error
                ),
        }
    }
}

impl Error for BrainfuckRuntimeError {}
