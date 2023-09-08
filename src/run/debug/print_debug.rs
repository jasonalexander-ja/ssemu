use baby_emulator::core::BabyModel;
use super::utils::{parse_memory_addresses, parse_registers};
use crate::args::{Registers, Run};
use crate::run::output::{output_model, output_all_registers, output_all_memory};
use crate::interface::Interface;


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


fn show_registers(regs: String, model: &BabyModel, int: &impl Interface) {
    let regs = regs.trim().to_owned();
    if regs.is_empty() {
        output_all_registers(model, int);
        return;
    }
    match parse_registers(regs.as_str()) {
        Ok(v) => output_model(&v, &vec![], false, model, int),
        Err(e) => int.log_warn(format!("Invalid register name: {}", e))
    }
}

fn show_memory_addresses(addrs: String, model: &BabyModel, int: &impl Interface) {
    let addrs = addrs.trim().to_owned();
    if addrs.is_empty() {
        output_all_memory(model, int);
        return;
    }
    match parse_memory_addresses(addrs) {
        Ok(v) => output_model(&vec![], &v, false, model, int),
        Err(e) => int.log_warn(format!("Invalid memory address: {}", e))
    }
}

fn print_addresses(addrs: &Vec<usize>, int: &impl Interface) {
    let addresses = addrs.iter()
        .map(|v| format!("{:#04x}", v))
        .collect::<Vec<String>>()
        .join(", ");
    int.log_msg(addresses);
}

fn print_registers(addrs: &Vec<Registers>, int: &impl Interface) {
    let regs = addrs.iter()
        .map(|v| format!("{:?}", v))
        .collect::<Vec<String>>()
        .join(", ");
    int.log_msg(regs);
}

pub fn print(command: String, conf: &Run, model: &BabyModel, int: &impl Interface) {
    let command = command.trim();
    match command.split(" ").next() {
        None => output_model(&conf.output_regs, &conf.output_addr, conf.output_model, model, int),
        Some("reg") => show_registers(command.replace("reg", ""), model, int),
        Some("mem") => show_memory_addresses(command.replace("mem", ""), model, int),
        Some("all-model") => output_model(&vec![], &vec![], true, model, int),
        Some("debug-addrs") => print_addresses(&conf.output_addr, int),
        Some("break-addrs") => print_addresses(&conf.break_addr, int),
        Some("debug-regs") => print_registers(&conf.output_regs, int),
        Some("help") => int.log_msg(format!("{}", PRINT_HELP)),
        _ => int.log_warn(format!("No recognised print command `{}`. \n {}", command, PRINT_HELP))
    }
}
