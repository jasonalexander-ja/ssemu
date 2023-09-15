use baby_emulator::core::BabyModel;
use colored::Colorize;
use crate::args::Run;
use crate::interface::Interface;
use super::output::output_model;
use debug_commands::match_debug_command;

/// Contains helpers for parsing/actioning debug commands. 
pub mod debug_commands;
/// Contains helpers for parsing/actioning print debug commands. 
pub mod print_debug;
/// Contains general helpers for debugging. 
pub mod utils;
/// Contains helpers for parsing/actioning modifying debug commands. 
pub mod modify;
#[cfg(test)]
mod tests;


/// Runs a debug session loop for the user for a givern model and configuration. 
/// 
/// Returns the model and config with any changes applied. 
/// 
/// # Parameters 
/// * `model` - The simulation model to run aganst. 
/// * `conf` - The configuration model to run against. 
/// * `int` - The interface used to i/o by the debug session. 
/// 
pub fn check_debug_session(model: &BabyModel, conf: &Run, int: &impl Interface) -> (BabyModel, Run) {
    let (mut model, mut conf) = (model.clone(), conf.clone());
    loop {
        int.log_msg(format!("{}", "Debug".cyan()));
        output_model(&conf.output_regs, &conf.output_addr, conf.output_model, &model, int);
        let line = int.get_line();
        if line.trim().starts_with("continue") { break; }
        (model, conf) = match_debug_command(line, &conf, &model, int);

    }

    (model, conf)
}
