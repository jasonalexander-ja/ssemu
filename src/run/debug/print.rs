use baby_emulator::core::BabyModel;
use super::utils::{parse_memory_addresses, parse_registers};
use crate::args::{Registers, Run};
use crate::run::output::{output_model, output_all_registers, output_all_memory};
use crate::interface::Interface;


/// The help message for a list printing commands. 
pub const PRINT_HELP: &str = 
"Possible sub-commands:

reg accumulator/instruction/instructionaddress - Outputs the registers
mem 0xA - Output a memory location (max 32, can be hex 0xA, decimal 10, octal 0o12, binary 0b1010)
all-model - Output the whole model
debug-addrs - The memory addresses to print upon debugging 
break-addrs - The memory addresses to enter debuging upon hitting 
debug-regs - The registers to print upon debugging
help - Print this help command";


/// Prints the model's registers based on a command string. 
/// 
/// Prints an error message if no register command is found. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being printed. 
/// * `model` - The model to be read. 
/// * `int` - The interface to print messages. 
/// 
pub fn show_registers(regs: String, model: &BabyModel, int: &impl Interface) {
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

/// Prints the model's memory addresses based on a command string. 
/// 
/// Prints an error message if command parsing fails. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being printed. 
/// * `model` - The model to be read. 
/// * `int` - The interface to print messages. 
/// 
pub fn show_memory_addresses(addrs: String, model: &BabyModel, int: &impl Interface) {
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

/// Prints a formatted list of memory address locations. 
/// 
/// # Parameters 
/// * `addrs` - The addresses to be printed. 
/// * `int` - The interface to print messages. 
/// 
pub fn print_addresses(addrs: &Vec<usize>, int: &impl Interface) {
    let addresses = addrs.iter()
        .map(|v| format!("{:#04x}", v))
        .collect::<Vec<String>>()
        .join(", ");
    int.log_msg(addresses);
}

/// Prints a formatted list of register names. 
/// 
/// # Parameters 
/// * `regs` - The registers to be printed. 
/// * `int` - The interface to print messages. 
/// 
pub fn print_registers(regs: &Vec<Registers>, int: &impl Interface) {
    let regs = regs.iter()
        .map(|v| format!("{:?}", v))
        .collect::<Vec<String>>()
        .join(", ");
    int.log_msg(regs);
}

/// Prints different parts of the model and run configuration based on a 
/// command string. 
/// 
/// Prints an error message if no matching command is found. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being printed. 
/// * `model` - The model to be read. 
/// * `conf` - The configuration model to be read. 
/// * `int` - The interface to print messages. 
/// 
pub fn print(command: String, conf: &Run, model: &BabyModel, int: &impl Interface) {
    let command = command.trim();
    let (next_com, _) = command.split_at(command.find(" ").unwrap_or(command.len()));
    let next_com = next_com.trim();
    match next_com {
        "reg" => show_registers(command.replace("reg", ""), model, int),
        "mem" => show_memory_addresses(command.replace("mem", ""), model, int),
        "all-model" => output_model(&vec![], &vec![], true, model, int),
        "debug-addrs" => print_addresses(&conf.output_addr, int),
        "break-addrs" => print_addresses(&conf.break_addr, int),
        "debug-regs" => print_registers(&conf.output_regs, int),
        "" | "h" | "help" => int.log_msg(format!("{}", PRINT_HELP)),
        _ => int.log_warn(format!("No recognised print command `{}`. \n{}", command, PRINT_HELP))
    }
}
