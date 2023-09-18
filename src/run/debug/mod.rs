use baby_emulator::core::BabyModel;
use colored::Colorize;
use crate::args::Run;
use crate::interface::Interface;
use super::output::output_model;
use commands::match_debug_command;

/// Contains helpers for parsing/actioning debug commands. 
pub mod commands;
/// Contains helpers for parsing/actioning print debug commands. 
pub mod print;
/// Contains general helpers for debugging. 
pub mod utils;
/// Contains helpers for parsing/actioning modifying debug commands. 
pub mod modify;
#[cfg(test)]
mod tests;


/// The result of a debug session, either it will exit, execute a single instruction
/// or continue. 
pub enum DebugResult {
    /// Continue execution. 
    Continue(BabyModel, Run),
    /// Perform the next instruction then debug. 
    SingleStep(BabyModel, Run),
    /// End execution and exit the emulator. 
    End(BabyModel, Run)
}

/// Runs a debug session loop for the user for a givern model and configuration. 
/// 
/// Returns the model and config with any changes applied. 
/// 
/// # Parameters 
/// * `model` - The simulation model to run aganst. 
/// * `conf` - The configuration model to run against. 
/// * `int` - The interface used to i/o by the debug session. 
/// 
pub fn check_debug_session(model: &BabyModel, conf: &Run, int: &impl Interface) -> DebugResult {
    let (mut model, mut conf) = (model.clone(), conf.clone());
    int.log_msg(format!("{}", "Debug".cyan()));
    output_model(&conf.output_regs, &conf.output_addr, conf.output_model, &model, int);
    loop {
        int.log_inline(format!("(ssemu-debug) "));

        let line = int.get_line()
            .to_lowercase()
            .trim()
            .to_owned();

        if line.starts_with("continue") || line.starts_with("c") { break; }
        if line.starts_with("next") || line.starts_with("n") 
            { return DebugResult::SingleStep(model, conf) } 
        if line.starts_with("end") || line.starts_with("e") 
            { return DebugResult::End(model, conf) } 

        (model, conf) = match_debug_command(line, &conf, &model, int);
    }

    DebugResult::Continue(model, conf)
}
