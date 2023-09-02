use baby_emulator::core::BabyModel;
use colored::Colorize;
use crate::args::{Run, Registers};
use super::utils::{parse_register, parse_memory_value, parse_instruction, parse_memory_address};


const SET_HELP_MSG: &str = 
"reg accumulator/instruction/instructionaddress 0x10 - Set a register to a given value. 
mem 0x10 0x10 - Set a memory location to a given value (address value). 
debug-addrs +/- 0x10 - Add/remove a memory address to print on debug. 
break-addrs +/- 0x10 - Add/remove a memory address to debug when hit. 
debug-regs +/- accumulator/instruction/instructionaddress - Add/remove a register to print on debug. 
help - Print this help message. 
";


fn show_yellow_error(msg: &str) {
    println!("{}", msg.yellow())
}

fn set_instruction_reg(value: &String, address: bool, model: &BabyModel) -> Result<BabyModel, String> {
    let value = value.trim();
    let value = parse_instruction(value).map_err(|e| format!("Invalid value: `{}`. ", e))?;
    Ok(BabyModel {
        main_store: model.main_store.clone(),
        accumulator: model.accumulator,
        instruction_address: if address { value } else { model.instruction_address },
        instruction: if !address { value } else { model.instruction },
    })
}

fn set_accumulator(value: &String, model: &BabyModel) -> Result<BabyModel, String> {
    let value = value.trim();
    let value = parse_memory_value(value).map_err(|e| format!("Invalid value: `{}`. ", e))?;
    Ok(BabyModel {
        main_store: model.main_store.clone(),
        accumulator: value,
        instruction_address: model.instruction_address,
        instruction: model.instruction,
    })
}

fn set_register(command: String, model: &BabyModel) -> Result<BabyModel, String> {
    let command: Vec<String> = command.trim()
        .split(" ")
        .map(|v| v.trim().to_owned())
        .collect();
    let (register, value) = 
        if let (Some(r), Some(v)) = (command.get(0), command.get(1)) { (r, v) }
        else {
            return Err(format!("Please specify a register and a value. "));
        };
    let register = parse_register(&register).map_err(|_| format!("No such register `{}`", register))?;
    match register {
        Registers::Accumulator => set_accumulator(value, model),
        Registers::Instruction => set_instruction_reg(value, false, model),
        Registers::InstructionAddress => set_instruction_reg(value, true, model),
    }

}

fn set_memory_address(command: String, model: &BabyModel) -> Result<BabyModel, String> {
    let command: Vec<String> = command.trim()
        .split(" ")
        .map(|v| v.trim().to_owned())
        .collect();
    let (address, value) = 
        if let (Some(r), Some(v)) = (command.get(0), command.get(1)) { (r, v) }
        else { return Err("Please specify a register and a value. ".to_owned()); };
    let address = parse_memory_address(&address).map_err(|e| format!("Invalid memory address `{}`. ", e))?;
    let value = parse_memory_value(&value).map_err(|e| format!("Invalid value: `{}`. ", e))?;
    let mut main_store = model.main_store.clone();
    main_store[address] = value;
    Ok(BabyModel {
        main_store,
        accumulator: model.accumulator,
        instruction_address: model.instruction_address,
        instruction: model.instruction
    })
}

fn remove_if_present<T: PartialEq>(value: T, vals: Vec<T>) -> Vec<T> {
    if vals.contains(&value) { vals }
    else {
        vals.into_iter().filter(|v| *v != value).collect()
    }
}

fn add_if_present<T: PartialEq + Clone>(value: T, vals: Vec<T>) -> Vec<T> {
    if vals.contains(&value) { vals }
    else {
        let mut vals = vals.clone();
        vals.push(value);
        vals
    }
}

fn add_or_remove<T: PartialEq + Clone>(action: &String, value: T, vals: Vec<T>) -> Result<Vec<T>, String> {
    match action.as_str() {
        "+" => Ok(add_if_present(value, vals)),
        "-" => Ok(remove_if_present(value, vals)),
        v => return Err(format!("Invalid action `{}`, actions must be add (+) or remove (-)", v))
    }
}

fn set_debug_address(command: String, conf: &Run) -> Result<Run, String> {
    let command: Vec<String> = command.trim()
        .split(" ")
        .map(|v| v.trim().to_owned())
        .collect();
    let (action, value) = 
        if let (Some(r), Some(v)) = (command.get(0), command.get(1)) { (r, v) }
        else { return Err("Please specify an action (either -/+) and an address. ".to_owned()); };

    let address = parse_memory_address(&value).map_err(|e| format!("Invalid memory address `{}`. ", e))?;
    let output_addr = add_or_remove(action, address, conf.output_addr.clone()).map_err(|e| e)?;
    let mut conf = conf.clone();
    conf.output_addr = output_addr;
    Ok(conf)
}

fn set_break_address(command: String, conf: &Run) -> Result<Run, String> {
    let command: Vec<String> = command.trim()
        .split(" ")
        .map(|v| v.trim().to_owned())
        .collect();
    let (action, value) = 
        if let (Some(r), Some(v)) = (command.get(0), command.get(1)) { (r, v) }
        else { return Err("Please specify an action (either -/+) and an address. ".to_owned()); };
        
    let address = parse_memory_address(&value).map_err(|e| format!("Invalid memory address `{}`. ", e))?;
    let break_addr = add_or_remove(action, address, conf.break_addr.clone()).map_err(|e| e)?;
    let mut conf = conf.clone();
    conf.break_addr = break_addr;
    Ok(conf)
}

fn set_debug_regs(command: String, conf: &Run) -> Result<Run, String> {
    let command: Vec<String> = command.trim()
        .split(" ")
        .map(|v| v.trim().to_owned())
        .collect();
    let (action, value) = 
        if let (Some(r), Some(v)) = (command.get(0), command.get(1)) { (r, v) }
        else { return Err("Please specify an action (either -/+) and an address. ".to_owned()); };

    let register = parse_register(&value).map_err(|e| format!("Invalid register name `{}`. ", e))?;
    let output_regs = add_or_remove(action, register, conf.output_regs.clone()).map_err(|e| e)?;
    let mut conf = conf.clone();
    conf.output_regs = output_regs;
    Ok(conf)
}

fn parse_set_model(command: &str, model: &BabyModel) -> Result<BabyModel, String> {
    let next_com = command.split(" ").next();
    let model = match &next_com {
        Some("reg") => set_register(command.replace("reg", ""), model).map_err(|e| e)?,
        Some("mem") => set_memory_address(command.replace("mem", ""), model).map_err(|e| e)?,
        _ => return Err(format!("No such option. "))
    };
    return Ok(model)
}

fn parse_set_config(command: &str, conf: &Run) -> Result<Run, String> {
    let next_com = command.split(" ").next();
    let model = match &next_com {
        Some("debug-addrs") => set_debug_address(command.replace("debug-addrs", ""), conf).map_err(|e| e)?,
        Some("break-addrs") => set_break_address(command.replace("break-addrs", ""), conf).map_err(|e| e)?,
        Some("debug-regs") => set_debug_regs(command.replace("debug-regs", ""), conf).map_err(|e| e)?,
        Some("") | Some("help") => { println!("{}", SET_HELP_MSG); conf.clone() },
        _ => return Err(format!("No recognised set command `{}`.", command))
    };
    return Ok(model)
}

pub fn parse_set_command(command: String, conf: &Run, model: &BabyModel) -> Result<(BabyModel, Run), String> {
    let command = command.trim();
    match parse_set_model(command, model) {
        Ok(m) => return Ok((m, conf.clone())),
        Err(e) => e
    };
    match parse_set_config(command, conf) {
        Ok(m) => return Ok((model.clone(), m.clone())),
        Err(e) => e
    };
    return Err(format!("No such option as `{}`. ", command));
}

pub fn modify(command: String, conf: &Run, model: &BabyModel) -> (BabyModel, Run) {
    match parse_set_command(command, conf, model) {
        Ok(v) => v,
        Err(e) => {
            show_yellow_error(e.as_str());
            (model.clone(), conf.clone())
        }
    }
}
