use bf_interp::VM;
use clap::Parser;
use std::process::exit;

mod cli;

fn run_bf(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    let program = bf_types::Program::from_file(args.program)?;
    program.validate()?;
    let virtual_machine: VM<u8> = VM::new(args.cells, args.extensible);
    virtual_machine.interpret(&program);
    Ok(())
}

/// The entry point for Brainfuck Interpreter. The program will treat the first command line argument as a path to a
/// brainfuck source code file, and interpret it.
/// # Example usage:
/// ```shell
/// cargo run -- --program=./hello_world.bf
/// ```
fn main() {
    let args = cli::Args::parse();
    let result = run_bf(args);
    if let Err(e) = result {
        eprintln!("{e}");
        exit(-1);
    }
}
