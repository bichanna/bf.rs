mod execution;
mod instruction;
mod interpreter;

use std::process::ExitCode;

pub use execution::ExecutionType;
pub use instruction::Instruction;
use interpreter::UnbalancedBrackets;

fn execute(source: &[u8], execution_type: ExecutionType) -> ExitCode {
    match execution_type {
        ExecutionType::Interpreter => match interpreter::interpret(source) {
            Ok(_) => ExitCode::from(0),
            Err(UnbalancedBrackets(c, addr)) => {
                eprintln!(
                    "Error parsing file: didn't find pair of '{}' at inst index {}",
                    c, addr
                );
                return ExitCode::from(3);
            }
        },
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args();
    if args.len() != 3 {
        eprintln!("Expected execution type and a file path");
        return ExitCode::from(1);
    }

    // Get execution type
    let execution_type = ExecutionType::from(args.nth(1).unwrap());

    // Get source code
    let filename = args.nth(0).unwrap();
    let source = match std::fs::read(&filename) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Error reading '{}': {}", filename, err);
            return ExitCode::from(2);
        }
    };

    execute(&source, execution_type)
}
