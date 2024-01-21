mod execution;
mod instruction;
mod program;

use std::process::ExitCode;

pub use execution::ExecutionType;
pub use instruction::Instruction;
pub use program::Program;
use program::UnbalancedBrackets;

fn execute(source: &[u8], execution_type: ExecutionType) -> ExitCode {
    let mut program = match Program::new(source) {
        Ok(x) => x,
        Err(UnbalancedBrackets(c, addr)) => {
            eprintln!(
                "Error parsing file: didn't find pair for '{}' at inst index {}",
                c, addr
            );
            return ExitCode::from(3);
        }
    };

    match execution_type {
        ExecutionType::Interpreter => match program.interpret() {
            Ok(_) => {}
            Err(err) => eprintln!("IO error: {}", err),
        },
    }

    ExitCode::from(0)
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
