use baby_emulator::core::MEMORY_WORDS;
use crate::args::Registers;


/// Parses a register name from a given string. 
/// 
/// Value can be `accumulator`, `instruction`, `instructionaddress` (case insensitive). 
/// 
/// # Parameters 
/// * `value` - The string containing a register name. 
/// 
/// # Returns 
/// * [Ok(Registers)] - The tokenised register name. 
/// * [Err(String)] - A formated error message if the value is incorrect. 
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

/// Parses register names from a given string. 
/// 
/// Values can be `accumulator`, `instruction`, `instructionaddress` (case insensitive),
/// and are commo separated. 
/// 
/// Uses [parse_register] internally.
/// 
/// # Parameters 
/// * `value` - The string containing a register names. 
/// 
/// # Returns 
/// * [Ok(Vec<Registers>)] - The tokenised register names. 
/// * [Err(String)] - A formated error message if the value is incorrect. 
pub fn parse_registers(input: &str) -> Result<Vec<Registers>, String> {
    let mut res = vec![];
    let regs: Vec<&str> = input.split(",").collect();
    for reg in regs {
        res.push(parse_register(reg).map_err(|e| e)?);
    }
    Ok(res)
}

/// Parses an memory address value from a given string. 
/// 
/// Value can be hex/decimal/octal/binary (prefixed with `0x`, `0d`, `0o`, or `0b`). 
/// Decimal doesn't need a prefix. 
/// 
/// # Parameters 
/// * `value` - The string containing a memory address value. 
/// 
/// # Returns 
/// * [Ok(usize)] - The memory address value. 
/// * [Err(String)] - A formated error message if the value is incorrect. 
pub fn parse_memory_address(value: &str) -> Result<usize, String> {
    let value = value.trim().to_lowercase();
    let parse_res = match value.clone() {
        v if v.starts_with("0x") => usize::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => usize::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => usize::from_str_radix(&v.replace("0b", ""), 2),
        v => usize::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value)?;
    if parse_res > MEMORY_WORDS {
        return Err(format!("Value passed output memory address: {parse_res}, Baby memory address space is {MEMORY_WORDS}. "));
    }
    Ok(parse_res)
}

/// Parses an memory value from a given string. 
/// 
/// Value can be hex/decimal/octal/binary (prefixed with `0x`, `0d`, `0o`, or `0b`). 
/// Decimal doesn't need a prefix. 
/// 
/// # Parameters 
/// * `value` - The string containing a memory value. 
/// 
/// # Returns 
/// * [Ok(i32)] - The memory value. 
/// * [Err(String)] - A formated error message if the value is incorrect. 
pub fn parse_memory_value(value: &str) -> Result<i32, String> {
    let value = value.trim().to_lowercase();
    let parse_res = match value.clone() {
        v if v.starts_with("0x") => i32::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => i32::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => i32::from_str_radix(&v.replace("0b", ""), 2),
        v => i32::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value)?;
    Ok(parse_res)
}

/// Parses a an instruction value from a given string. 
/// 
/// Value can be hex/decimal/octal/binary (prefixed with `0x`, `0d`, `0o`, or `0b`). 
/// Decimal doesn't need a prefix. 
/// 
/// # Parameters 
/// * `value` - The string containing an instruction value. 
/// 
/// # Returns 
/// * [Ok(u16)] - The instruction value. 
/// * [Err(String)] - A formated error message if the value is incorrect. 
/// 
pub fn parse_instruction(value: &str) -> Result<u16, String> {
    let value = value.trim().to_lowercase();
    let parse_res = match value.clone() {
        v if v.starts_with("0x") => u16::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => u16::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => u16::from_str_radix(&v.replace("0b", ""), 2),
        v => u16::from_str_radix(&v.replace("0d", ""), 10),
    }.map_err(|_| value)?;
    Ok(parse_res)
}

/// Parses a list of memory address location values from a given string. 
/// 
/// Parses from a list of comma separated values. 
/// See [parse_memory_address] for a list of possible value formats. 
/// 
/// # Parameters 
/// * `addresses` - The string containing a list of memory locations. 
/// 
/// # Returns 
/// * [Ok(Vec<usize>)] - A vector of memory locations. 
/// * [Err(String)] - A formated error message if a value is incorrect. 
/// 
pub fn parse_memory_addresses(addresses: String) -> Result<Vec<usize>, String> {
    let addr_values = addresses.trim().split(",");
    let mut addresses: Vec<usize> = vec![];
    for addr in addr_values {
        let res = parse_memory_address(addr)
            .map_err(|e| format!("Invalid value passed as output memory address `{e}`. "))?;
        addresses.push(res);
    }
    Ok(addresses)
}
