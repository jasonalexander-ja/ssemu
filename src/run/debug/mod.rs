use std::io;
use baby_emulator::core::BabyModel;
use colored::Colorize;
use crate::args::Run;
use super::output::output_model;
use debug_commands::match_debug_command;

pub mod debug_commands;


pub fn check_debug_session(model: &BabyModel, conf: &Run) {
    loop {
        println!("{}", "Debug".cyan());
        output_model(&conf.output_regs, &conf.output_addr, conf.output_model, model);
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        match_debug_command(line, conf, model);

    }
}
