use baby_emulator::core::BabyModel;
use crate::args::{Run, Registers};
use crate::interface::Interface;
use super::utils::{parse_register, parse_memory_value, parse_instruction, parse_memory_address};


/// The help message printed for a list of set commands. 
pub const SET_HELP_MSG: &str = 
"Possible sub-commands:

reg accumulator/instruction/instructionaddress 0x10 - Set a register to a given value. 
mem 0x10 0x10 - Set a memory location to a given value (address value). 
debug-addrs +/- 0x10 - Add/remove a memory address to print on debug. 
break-addrs +/- 0x10 - Add/remove a memory address to debug when hit. 
debug-regs +/- accumulator/instruction/instructionaddress - Add/remove a register to print on debug. 
h, help - Print this help message. ";


/// Sets the intruction/address register of the passed model to the value to be parsed. 
/// 
/// # Parameters
/// * `value` - The value to be parsed. 
/// * `address` - Whether to set the address register (true) or the instruction register. 
/// * `model` - The model to be acted upon. 
/// 
/// # Returns
/// * [Ok(BabyModel)] - If the parsing and setting has happened correctly. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_instruction_reg(value: &String, address: bool, model: &BabyModel) -> Result<BabyModel, String> {
    let value = value.trim();
    let value = parse_instruction(value).map_err(|e| format!("Invalid value: `{}`. ", e))?;
    Ok(BabyModel {
        main_store: model.main_store.clone(),
        accumulator: model.accumulator,
        instruction_address: if address { value } else { model.instruction_address },
        instruction: if !address { value } else { model.instruction },
    })
}

/// Sets the accumulator to a parsed value. 
/// 
/// # Parameters 
/// * `value` - The string value to be parsed. 
/// * `model` - The model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(BabyModel)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_accumulator(value: &String, model: &BabyModel) -> Result<BabyModel, String> {
    let value = value.trim();
    let value = parse_memory_value(value).map_err(|e| format!("Invalid value: `{}`. ", e))?;
    Ok(BabyModel {
        main_store: model.main_store.clone(),
        accumulator: value,
        instruction_address: model.instruction_address,
        instruction: model.instruction,
    })
}

/// Sets a given register from parsing a command. 
/// 
/// # Parameters 
/// * `command` - The string command containing the register and the value to set it to. 
/// * `model` - The model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(BabyModel)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_register(command: String, model: &BabyModel) -> Result<BabyModel, String> {
    let command = command.trim();

    let index = if let Some(v) = command.find(" ") { v }
    else { return Err(format!("Please specify a register and a value. ")); };

    let (register, value) = command.split_at(index);

    let register = parse_register(&register)
        .map_err(|_| format!("No such register `{}`", register))?;
    match register {
        Registers::Accumulator => set_accumulator(&value.to_owned(), model),
        Registers::Instruction => set_instruction_reg(&value.to_owned(), false, model),
        Registers::InstructionAddress => set_instruction_reg(&value.to_owned(), true, model),
    }
}

/// Sets a given memory address to a given value from a parsed command string. 
/// 
/// # Parameters 
/// * `command` - The string command containing the memory location and the value to set it to. 
/// * `model` - The model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(BabyModel)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_memory_address(command: String, model: &BabyModel) -> Result<BabyModel, String> {
    let command = command.trim();

    let index = if let Some(v) = command.find(" ") { v }
    else { return Err(format!("Please specify a memory address and a value. ")); };

    let (address, value) = command.split_at(index);

    let address = parse_memory_address(&address)
        .map_err(|e| format!("Invalid memory address: `{}`. ", e))?;
    let value = parse_memory_value(&value)
        .map_err(|e| format!("Invalid value: `{}`. ", e))?;

    let mut main_store = model.main_store.clone();
    main_store[address] = value;
    Ok(BabyModel {
        main_store,
        accumulator: model.accumulator,
        instruction_address: model.instruction_address,
        instruction: model.instruction
    })
}

/// Helper function, removes a value from an array if it's present. 
/// 
/// # Parameters
/// * `value` - The value to be removed. 
/// * `vals` - The vector to be acted upon. 
/// 
/// # Returns 
/// A vec without the given value. 
/// 
pub fn remove_if_present<T: PartialEq>(value: T, vals: Vec<T>) -> Vec<T> {
    if !vals.contains(&value) { vals }
    else {
        vals.into_iter().filter(|v| *v != value).collect()
    }
}

/// Helper function, adds a value from an array if it's present. 
/// 
/// # Parameters
/// * `value` - The value to be added. 
/// * `vals` - The vector to be acted upon. 
/// 
/// # Returns 
/// A vec with the given value. 
/// 
pub fn add_if_not_present<T: PartialEq + Clone>(value: T, vals: Vec<T>) -> Vec<T> {
    if vals.contains(&value) { vals }
    else {
        let mut vals = vals.clone();
        vals.push(value);
        vals
    }
}

/// Adds or removes a value from a vector depending on a string command. 
/// 
/// # Parameters
/// * `action` - The string command, either "+" or "-" for add/remove. 
/// * `value` - The value to be added/removed. 
/// * `vals` - The vec to be acted upon. 
/// 
/// # Return 
/// * [Ok(Vec<T>)] - If sucessfully parsed the command string. 
/// * [Err(String)] - If there was an error parsing the command string, contains an error message. 
/// 
pub fn add_or_remove<T: PartialEq + Clone>(action: &String, value: T, vals: Vec<T>) -> Result<Vec<T>, String> {
    match action.as_str() {
        "+" => Ok(add_if_not_present(value, vals)),
        "-" => Ok(remove_if_present(value, vals)),
        v => return Err(format!("Invalid action `{}`, actions must be add (+) or remove (-)", v))
    }
}

/// Adds or removes an address to print upon debugging from a string command. 
/// 
/// # Parameters 
/// * `command` - The string command containing the memory location and whether to add or remove. 
/// * `conf` - The configuration model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(Run)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_debug_address(command: String, conf: &Run) -> Result<Run, String> {
    let command = command.trim();

    let index = if let Some(v) = command.find(" ") { v }
    else { return Err("Please specify an action (either -/+) and an address. ".to_owned()); };

    let (action, value) = command.split_at(index);

    let address = parse_memory_address(&value).map_err(|e| format!("Invalid memory address `{}`. ", e))?;
    let output_addr = add_or_remove(&action.to_owned(), address, conf.output_addr.clone()).map_err(|e| e)?;
    let mut conf = conf.clone();
    conf.output_addr = output_addr;
    Ok(conf)
}

/// Adds or removes an address to break into debug when hit, 
/// parsed from a string command. 
/// 
/// # Parameters 
/// * `command` - The string command containing the memory location and whether to add or remove. 
/// * `conf` - The configuration model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(Run)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_break_address(command: String, conf: &Run) -> Result<Run, String> {
    let command = command.trim();

    let index = if let Some(v) = command.find(" ") { v }
    else { return Err("Please specify an action (either -/+) and an address. ".to_owned()); };

    let (action, value) = command.split_at(index);
        
    let address = parse_memory_address(&value).map_err(|e| format!("Invalid memory address `{}`. ", e))?;
    let break_addr = add_or_remove(&action.to_owned(), address, conf.break_addr.clone()).map_err(|e| e)?;
    let mut conf = conf.clone();
    conf.break_addr = break_addr;
    Ok(conf)
}

/// Adds or removes a register to print upon debugging from a string command. 
/// 
/// # Parameters 
/// * `command` - The string command containing the register and whether to add or remove. 
/// * `conf` - The configuration model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(Run)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn set_debug_regs(command: String, conf: &Run) -> Result<Run, String> {
    let command = command.trim();

    let index = if let Some(v) = command.find(" ") { v }
    else { return Err("Please specify an action (either -/+) and an address. ".to_owned()); };

    let (action, value) = command.split_at(index);

    let register = parse_register(&value).map_err(|e| format!("Invalid register name `{}`. ", e))?;
    let output_regs = add_or_remove(&action.to_owned(), register, conf.output_regs.clone()).map_err(|e| e)?;
    let mut conf = conf.clone();
    conf.output_regs = output_regs;
    Ok(conf)
}

/// Checks to see if a set command is for the emulation model and dispatches the 
/// relevant set command. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being set (either "reg" or "mem"). 
/// * `model` - The model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(BabyModel)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn parse_set_model(command: &str, model: &BabyModel) -> Result<BabyModel, String> {
    let command = command.trim();
    let (next_com, _) = command.split_at(command.find(" ").unwrap_or(command.len() - 1));
    let next_com = next_com.trim();
    let model = match next_com {
        "reg" => set_register(command.replace("reg", ""), model).map_err(|e| e)?,
        "mem" => set_memory_address(command.replace("mem", ""), model).map_err(|e| e)?,
        _ => return Err(format!("No such option. "))
    };
    return Ok(model);
}

/// Checks to see if a set command is for the run configuration model and dispatches the 
/// relevant set command. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being set. 
/// * `conf` - The configuration model to be acted upon. 
/// 
/// # Returns 
/// * [Ok(Run)] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn parse_set_config(command: &str, conf: &Run, int: &impl Interface) -> Result<Run, String> {
    let command = command.trim();
    let (next_com, _) = command.split_at(command.find(" ").unwrap_or(command.len()));
    let next_com = next_com.trim();
    let model = match next_com {
        "debug-addrs" => set_debug_address(command.replace("debug-addrs", ""), conf).map_err(|e| e)?,
        "break-addrs" => set_break_address(command.replace("break-addrs", ""), conf).map_err(|e| e)?,
        "debug-regs" => set_debug_regs(command.replace("debug-regs", ""), conf).map_err(|e| e)?,
        "" | "h" | "help" => { int.log_msg(format!("{}", SET_HELP_MSG)); conf.clone() },
        _ => return Err(format!("No recognised set command `{}`.", command))
    };
    return Ok(model)
}

/// Parses a set command, tries to dispatch the relevant command. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being set. 
/// * `model` - The model to be acted upon. 
/// * `conf` - The configuration model to be acted upon. 
/// * `int` - The interface to print help messages. 
/// 
/// # Returns 
/// * [Ok((BabyModel, Run))] - If the parsing and setting suceeded. 
/// * [Err(String)] - If the parsing failed, contains an error message. 
/// 
pub fn parse_set_command(
    command: String, 
    conf: &Run, 
    model: &BabyModel, 
    int: &impl Interface
) -> Result<(BabyModel, Run), String> {
    let command = command.trim();
    match parse_set_model(command, model) {
        Ok(m) => return Ok((m, conf.clone())),
        Err(e) => e
    };
    match parse_set_config(command, conf, int) {
        Ok(m) => return Ok((model.clone(), m.clone())),
        Err(e) => e
    };
    return Err(format!("No such option as `{}`. ", command));
}

/// Parses a set command, tries to dispatch the relevant command, prints 
/// help messages and erros if failed.
/// 
/// Returns the updated model and configuration, will be the same if 
/// parsing failed.  
/// 
/// # Parameters 
/// * `command` - The string command stating what is being set. 
/// * `model` - The model to be acted upon. 
/// * `conf` - The configuration model to be acted upon. 
/// * `int` - The interface to print messages. 
/// 
pub fn modify(command: String, conf: &Run, model: &BabyModel, int: &impl Interface) -> (BabyModel, Run) {
    match parse_set_command(command, conf, model, int) {
        Ok(v) => v,
        Err(e) => {
            int.log_warn(e);
            (model.clone(), conf.clone())
        }
    }
}
