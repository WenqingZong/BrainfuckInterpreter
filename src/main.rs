use bf_interp::VM;
use std::{env, num::NonZeroUsize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).ok_or("Please specify an input file")?;
    let program = bf_types::Program::from_file(&file_path)?;
    let virtual_machine: VM<u8> = VM::new(NonZeroUsize::new(0), false);
    println!("{virtual_machine:?}");
    virtual_machine.interpret(&program);
    Ok(())
}
