use baby_emulator::core::MEMORY_WORDS;
use crate::args::Registers;


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

pub fn parse_registers(input: &str) -> Result<Vec<Registers>, String> {
    let mut res = vec![];
    let regs: Vec<&str> = input.split(",").collect();
    for reg in regs {
        res.push(parse_register(reg).map_err(|e| e)?);
    }
    Ok(res)
}

pub fn parse_memory_address(value: &str) -> Result<usize, String> {
    let parse_res = match value {
        v if v.starts_with("0x") => usize::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => usize::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => usize::from_str_radix(&v.replace("0b", ""), 2),
        v => usize::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value.to_owned())?;
    if parse_res > MEMORY_WORDS {
        return Err(format!("Value passed output memory address: {parse_res}, Baby memory address space is {MEMORY_WORDS}. "));
    }
    Ok(parse_res)
}

pub fn parse_memory_value(value: &str) -> Result<i32, String> {
    let parse_res = match value {
        v if v.starts_with("0x") => i32::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => i32::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => i32::from_str_radix(&v.replace("0b", ""), 2),
        v => i32::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value.to_owned())?;
    Ok(parse_res)
}

pub fn parse_instruction(value: &str) -> Result<u16, String> {
    let parse_res = match value {
        v if v.starts_with("0x") => u16::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => u16::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => u16::from_str_radix(&v.replace("0b", ""), 2),
        v => u16::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value.to_owned())?;
    Ok(parse_res)
}

pub fn parse_memory_addresses(addresses: String) -> Result<Vec<usize>, String> {
    let addr_values = addresses.trim().split(",");
    let mut addresses: Vec<usize> = vec![];
    for addr in addr_values {
        let res = parse_memory_address(addr)
            .map_err(|_| format!("Invalid value passed as output memory address `{addr}`. "))?;
        addresses.push(res);
    }
    Ok(addresses)
}
