use baby_emulator::core::BabyModel;
use colored::Colorize;
use super::utils::{parse_memory_addresses, parse_registers};
use crate::args::{Registers, Run};
use crate::run::output::{output_model, output_all_registers, output_all_memory};


const PRINT_HELP: &str = 
"Possible commands:
reg accumulator/instruction/instructionaddress - Outputs the registers
mem 0xA - Output a memory location (max 32, can be hex 0xA, decimal 10, octal 0o12, binary 0b1010)
all-model - Output the whole model
debug-addrs - The memory addresses to print upon debugging 
break-addrs - The memory addresses to enter debuging upon hitting 
debug-regs - The registers to print upon debugging
help - Print this help command
";

fn show_yellow_error(msg: &str) {
    println!("{}", msg.yellow())
}

fn show_registers(regs: String, model: &BabyModel) {
    let regs = regs.trim().to_owned();
    if regs.is_empty() {
        output_all_registers(model);
        return;
    }
    match parse_registers(regs.as_str()) {
        Ok(v) => output_model(&v, &vec![], false, model),
        Err(e) => show_yellow_error(format!("Invalid register name: {}", e).as_str())
    }
}

fn show_memory_addresses(addrs: String, model: &BabyModel) {
    let addrs = addrs.trim().to_owned();
    if addrs.is_empty() {
        output_all_memory(model);
        return;
    }
    match parse_memory_addresses(addrs) {
        Ok(v) => output_model(&vec![], &v, false, model),
        Err(e) => show_yellow_error(format!("Invalid memory address: {}", e).as_str())
    }
}

fn print_addresses(addrs: &Vec<usize>) {
    addrs.iter().for_each(|v| print!("{:#04x}", v));
    println!("");
}

fn print_registers(addrs: &Vec<Registers>) {
    addrs.iter().for_each(|v| print!("{:?}", v));
    println!("");
}

pub fn print(command: String, conf: &Run, model: &BabyModel) {
    let command = command.trim();
    match command.split(" ").next() {
        None => output_model(&conf.output_regs, &conf.output_addr, conf.output_model, model),
        Some("reg") => show_registers(command.replace("reg", ""), model),
        Some("mem") => show_memory_addresses(command.replace("mem", ""), model),
        Some("all-model") => output_model(&vec![], &vec![], true, model),
        Some("debug-addrs") => print_addresses(&conf.output_addr),
        Some("break-addrs") => print_addresses(&conf.break_addr),
        Some("debug-regs") => print_registers(&conf.output_regs),
        Some("help") => println!("{}", PRINT_HELP),
        _ => show_yellow_error(format!("No recognised print command `{}`. \n {}", command, PRINT_HELP).as_str())
    }
}
