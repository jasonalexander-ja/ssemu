use baby_emulator::core::MEMORY_WORDS;
use crate::interface::Interface;
use super::args::Run;
use super::errors::Errors;
use srcfile::get_src;
use execution::run_model;

/// Contains possible errors encountered during runtime. 
pub mod errors;
/// Contains helpers for retreiving the source data. 
pub mod srcfile;
/// Contains helpers for outputting a model. 
pub mod output;
/// Contains helpers for executing a model. 
pub mod execution;
/// Contains helpers for debugging a model. 
pub mod debug;
#[cfg(test)]
mod tests;


/// A type helper for a program stack. 
pub type ProgramStack = [i32; MEMORY_WORDS];

/// Gets a program stack from the source and executes it based on given configuration. 
/// 
/// # Parameters 
/// * `args` - The configuration to be used. 
/// * `interface` - The interface to read and write to. 
/// 
pub fn execute(args: Run, interface: &impl Interface) -> Result<(), Errors> {
    let src = get_src(&args, interface).map_err(|e| Errors::RuntimeError(e))?;
    run_model(args, src, interface);

    Ok(())
}
