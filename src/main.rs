use bf_interp::VM;
use clap::Parser;

mod cli;

/// The entry point for Brainfuck Interpreter. The program will treat the first command line argument as a path to a
/// brainfuck source code file, and interpret it.
/// # Example usage:
/// ```shell
/// cargo run -- --program=./hello_world.bf
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    let program = bf_types::Program::from_file(args.program)?;
    let virtual_machine: VM<u8> = VM::new(args.cells, args.extensible);
    virtual_machine.interpret(&program);
    Ok(())
}
