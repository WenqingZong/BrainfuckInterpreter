use bf_interp::VM;
use clap::Parser;
use std::process::ExitCode;

mod cli;

fn run_bf(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    let program = bf_types::Program::from_file(args.program)?;
    program.validate()?;
    let virtual_machine: VM<u8> = VM::new(args.cells, args.extensible);
    virtual_machine.interpret(&program);
    Ok(())
}

/// The entry point for Brainfuck Interpreter. The program has a modern CLI, which contains everything you should know.
/// # Example:
/// ```shell
/// # To get more detailed help with this tool.
/// cargo run -- --help
///
/// # To actually interpret a Brainfuck program.
/// cargo run -- ./hello_world.bf
/// ```
fn main() -> ExitCode {
    let args = cli::Args::parse();
    let result = run_bf(args);
    if let Err(e) = result {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
