use bf_interp::VM;
use std::{env, num::NonZeroUsize};

/// The entry point for Brainfuck Interpreter. The program will treat the first command line argument as a path to a
/// brainfuck source code file, and interpret it.
/// # Example usage:
/// ```shell
/// cargo run -- --./hello_world.bf
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).ok_or("Please specify an input file")?;
    let program = bf_types::Program::from_file(file_path)?;
    let virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(10), false);
    println!("{virtual_machine:?}");
    virtual_machine.interpret(&program);
    Ok(())
}
