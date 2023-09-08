use baby_emulator::core::MEMORY_WORDS;
use crate::interface::Interface;
use super::args::Run;
use super::errors::Errors;
use srcfile::get_src;
use execution::run_model;

pub mod errors;
pub mod srcfile;
pub mod output;
pub mod execution;
pub mod debug;


pub type ProgramStack = [i32; MEMORY_WORDS];

pub fn execute(args: Run, interface: &impl Interface) -> Result<(), Errors> {
    let src = get_src(&args).map_err(|e| Errors::RuntimeError(e))?;
    run_model(args, src, interface);

    Ok(())
}
