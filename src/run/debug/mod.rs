use std::io;
use baby_emulator::core::BabyModel;
use colored::Colorize;
use crate::args::Run;
use super::output::output_model;
use debug_commands::match_debug_command;

pub mod debug_commands;
pub mod print_debug;
pub mod utils;
pub mod modify;


pub fn check_debug_session(model: &BabyModel, conf: &Run) -> (BabyModel, Run) {
    let (mut model, mut conf) = (model.clone(), conf.clone());
    loop {
        println!("{}", "Debug".cyan());
        output_model(&conf.output_regs, &conf.output_addr, conf.output_model, &model);
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        if line.trim().starts_with("continue") { break; }
        (model, conf) = match_debug_command(line, &conf, &model);

    }

    (model, conf)
}
