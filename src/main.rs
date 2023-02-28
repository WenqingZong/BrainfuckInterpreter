use bf_types;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).ok_or("Please specify an input file")?;
    let program = bf_types::Program::from_file(&file_path)?;
    println!("{}", program);
    Ok(())
}
