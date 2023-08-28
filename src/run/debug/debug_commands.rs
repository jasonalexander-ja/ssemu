use baby_emulator::core::{MEMORY_WORDS, BabyModel};
use colored::Colorize;
use crate::args::{Registers, Run};
use crate::run::output::{output_model, output_all_registers, output_all_memory};


const HELP: &str = 
"Possible commands:
print - Print the value of a register or memory location(s)  
help - Print this help command
";

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

pub fn parse_register(input: &str) -> Result<Registers, String> {
    let input = input.to_lowercase();
    let input = input.trim();
    match input {
        "accumulator" => Ok(Registers::Accumulator),
        "instruction" => Ok(Registers::Instruction),
        "instructionaddress" => Ok(Registers::InstructionAddress),
        _ => Err(input.to_string())
    }
}

fn parse_memory_address(value: &str) -> Result<usize, String> {
    let parse_res = match value {
        v if v.starts_with("0x") => usize::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => usize::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => usize::from_str_radix(&v.replace("0b", ""), 2),
        v => usize::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value.to_owned())?;
    Ok(parse_res)
}

pub fn parse_memory_addresses(addresses: String) -> Result<Vec<usize>, String> {
    let addr_values = addresses.trim().split(",");
    let mut addresses: Vec<usize> = vec![];
    for addr in addr_values {
        let res = parse_memory_address(addr)
            .map_err(|_| format!("Invalid value passed as output memory address `{addr}`. "))?;
        if res > MEMORY_WORDS {
            return Err(format!("Value passed output memory address: {res}, Baby memory address space is {MEMORY_WORDS}. "));
        }
        addresses.push(res);
    }
    Ok(addresses)
}

fn parse_registers(input: &str) -> Result<Vec<Registers>, String> {
    let mut res = vec![];
    let regs: Vec<&str> = input.split(",").collect();
    for reg in regs {
        res.push(parse_register(reg).map_err(|e| e)?);
    }
    Ok(res)
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
        Err(e) => show_yellow_error(format!("Invalid register name: {}", e).as_str())
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

fn print(command: String, conf: &Run, model: &BabyModel) {
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

fn set(command: String, conf: &Run, model: &BabyModel) {
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
        _ => show_yellow_error(format!("No recognised set command `{}`. \n {}", command, PRINT_HELP).as_str())
    }
}

pub fn match_debug_command(command: String, conf: &Run, model: &BabyModel) {
    let command = command.trim();
    match command.split(" ").next() {
        Some("print") => print(command.replace("print", ""), conf, model),
        Some("help") => println!("{}", HELP),
        _ => ()
    }
}
