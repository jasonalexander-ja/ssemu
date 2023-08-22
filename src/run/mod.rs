use baby_emulator::core::MEMORY_WORDS;
use super::args::Run;
use errors::RunError;
use srcfile::get_src;
use execution::run_model;

pub mod errors;
pub mod srcfile;
pub mod output;
pub mod execution;
pub mod debug;


pub type ProgramStack = [i32; MEMORY_WORDS];

pub fn execute(args: Run) -> Result<(), RunError> {
    let src = get_src(&args)?;
    run_model(args, src);

    Ok(())
}
