use std::io;
use baby_emulator::core::BabyModel;
use baby_emulator::core::errors::BabyErrors;
use colored::Colorize;
use crate::args::Run;
use crate::args::Registers;
use super::output::output_model;


fn show_register(reg: &str, model: &BabyModel) {
    
}

fn show_registers(regs: &str, model: &BabyModel) {
    match  Registers::parse_registers(regs) {
        Ok(v) => output_model(&v, &vec![], false, model),
        Err(e) => println!()
    }
}


fn match_debug_command(command: String, model: &BabyModel) {
    match command {
        v if v.starts_with("reg") => show_register(v.as_str(), model)
    }
}

pub fn check_debug_session(model: &BabyModel, conf: &Run) {
    loop {
        println!("{}", "Debug".cyan());
        output_model(&conf.output_regs, &conf.output_addr, conf.output_model, model);
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        

    }
}
