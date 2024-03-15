use std::path::PathBuf;
use super::modify;
use crate::args::{Run, ExecuteFrom, Registers};
use crate::test_utils::TestInterface;
use baby_emulator::core::BabyModel;



#[test]
fn test_set_instruction_reg() {
    let model = BabyModel::new();

    let res = modify::set_instruction_reg(&format!("   0xA   "), false, &model);
    match res {
        Ok(v) => assert_eq!(v.instruction, 0xA),
        Err(_) => panic!("Parsing IR set command failed. ")
    }

    let res = modify::set_instruction_reg(&format!("   0xJ   "), false, &model);
    match res {
        Err(v) => assert!(v.starts_with("Invalid value:")),
        Ok(_) => panic!("Parsing invalid IR set command suceeded. ")
    }
}

#[test]
fn test_set_address_instruction_reg() {
    let model = BabyModel::new();

    let res = modify::set_instruction_reg(&format!("   0xA   "), true, &model);
    match res {
        Ok(v) => assert_eq!(v.instruction_address, 0xA),
        Err(_) => panic!("Parsing IR address set command failed. ")
    }

    let res = modify::set_instruction_reg(&format!("   0xJ   "), true, &model);
    match res {
        Err(v) => assert!(v.starts_with("Invalid value:")),
        Ok(_) => panic!("Parsing invalid IR address set command suceeded. ")
    }
}

#[test]
fn test_set_accumulator() {
    let model = BabyModel::new();

    let res = modify::set_accumulator(&format!("   0xA   "), &model);
    match res {
        Ok(v) => assert_eq!(v.accumulator, 0xA),
        Err(_) => panic!("Parsing accumulator set command failed. ")
    }

    let res = modify::set_accumulator(&format!("   0xJ   "), &model);
    match res {
        Err(v) => assert!(v.starts_with("Invalid value:")),
        Ok(_) => panic!("Parsing invalid accumulator set command suceeded. ")
    }
}

#[test]
fn test_set_register() {
    let model = BabyModel::new();

    let res = modify::set_register(format!("   accumulator   0xA   "), &model);
    assert!(res.is_ok());

    let res = modify::set_register(format!("   accumulator   "), &model);
    if let Err(e) = res { assert_eq!(e, format!("Please specify a register and a value. ")) }
    else { panic!("Incorrect number of args into register set suceeded. ") }

    let res = modify::set_register(format!("   foo   bbb   "), &model);
    if let Err(e) = res { assert!(e.starts_with("No such register")) }
    else { panic!("Incorrect number of args into register set suceeded. ") }
}

#[test]
fn test_set_memory_address() {
    let model = BabyModel::new();

    let res = modify::set_memory_address(format!("   0xA   0xA   "), &model);
    assert!(res.is_ok());

    let res = modify::set_memory_address(format!("   0xA   "), &model);
    if let Err(e) = res { assert_eq!(e, format!("Please specify a memory address and a value. ")) }
    else { panic!("Incorrect number of args into memory set suceeded. ") }

    let res = modify::set_memory_address(format!("   foo   0xA   "), &model);
    if let Err(e) = res { assert!(e.starts_with("Invalid memory address:")) }
    else { panic!("Incorrect address arg into memory set suceeded. ") }

    let res = modify::set_memory_address(format!("   0xA   foo   "), &model);
    if let Err(e) = res { assert!(e.starts_with("Invalid value:")) }
    else { panic!("Incorrect value arg into memory set suceeded. ") }
}

#[test]
fn test_add_if_present() {
    assert_eq!(modify::add_if_not_present(5, vec![]), vec![5]);
    assert_eq!(modify::add_if_not_present(5, vec![5]), vec![5]);
}

#[test]
fn test_remove_if_present() {
    assert_eq!(modify::remove_if_present(5, vec![5]), Vec::<i32>::new());
    assert_eq!(modify::remove_if_present(5, vec![6]), vec![6]);
}

#[test]
fn test_add_or_remove() {
    assert_eq!(modify::add_or_remove(&format!("+"), 5, vec![]), Ok(vec![5]));
    assert_eq!(modify::add_or_remove(&format!("+"), 5, vec![5]), Ok(vec![5]));
    assert_eq!(modify::add_or_remove(&format!("-"), 5, vec![5]), Ok(vec![]));
    assert_eq!(modify::add_or_remove(&format!("-"), 5, vec![6]), Ok(vec![6]));
    match modify::add_or_remove(&format!("foo"), 5, vec![]) {
        Err(v) => assert!(v.starts_with("Invalid action")),
        Ok(_) => panic!("Add or remove suceeded with invalid action. ")
    }
}

fn default_run() -> Run {
    Run {
        src: PathBuf::from("foo"),
        exe_from: ExecuteFrom::Bin,
        og_notation: false,
        output_model: false,
        debug_on_err: true,
        output_addr: vec![5],
        output_regs: vec![Registers::Accumulator],
        break_addr: vec![6]
    }
}

#[test]
fn test_set_debug_address() {
    let conf = default_run();
    match modify::set_debug_address(format!("   +   0xA   "), &conf) {
        Ok(c) => assert_eq!(c.output_addr, vec![5, 10]),
        Err(_) => panic!("Valid set debug address failed. ")
    }
    match modify::set_debug_address(format!("   0xA   "), &conf) {
        Err(e) => assert!(e.starts_with("Please specify an action (either -/+) and an address.")),
        Ok(_) => panic!("Invalid no of args set debug address suceeded. ")
    }
    match modify::set_debug_address(format!("   +   0xH   "), &conf) {
        Err(e) => assert!(e.starts_with("Invalid memory address")),
        Ok(_) => panic!("Invalid args set debug address suceeded. ")
    }
    assert!(modify::set_debug_address(format!("   q   0xA   "), &conf).is_err());
}

#[test]
fn test_set_break_address() {
    let conf = default_run();
    match modify::set_break_address(format!("   +   0xA   "), &conf) {
        Ok(c) => assert_eq!(c.break_addr, vec![6, 10]),
        Err(_) => panic!("Valid set debug address failed. ")
    }
    match modify::set_break_address(format!("   0xA   "), &conf) {
        Err(e) => assert!(e.starts_with("Please specify an action (either -/+) and an address.")),
        Ok(_) => panic!("Invalid no of args set debug address suceeded. ")
    }
    match modify::set_break_address(format!("   +   0xH   "), &conf) {
        Err(e) => assert!(e.starts_with("Invalid memory address")),
        Ok(_) => panic!("Invalid args set debug address suceeded. ")
    }
    assert!(modify::set_break_address(format!("   q   0xA   "), &conf).is_err());
}

#[test]
fn test_set_debug_regs() {
    let conf = default_run();
    match modify::set_debug_regs(format!("   +    instruction   "), &conf) {
        Ok(c) => assert_eq!(c.output_regs, vec![Registers::Accumulator, Registers::Instruction]),
        Err(_) => panic!("Valid set debug address failed. ")
    }
    match modify::set_debug_regs(format!("   instruction   "), &conf) {
        Err(e) => assert!(e.starts_with("Please specify an action (either -/+) and an address.")),
        Ok(_) => panic!("Invalid no of args set debug address suceeded. ")
    }
    match modify::set_debug_regs(format!("    +   foo    "), &conf) {
        Err(e) => assert!(e.starts_with("Invalid register name")),
        Ok(_) => panic!("Invalid args set debug address suceeded. ")
    }
    assert!(modify::set_debug_regs(format!("   q   instruction   "), &conf).is_err());
}

#[test]
fn test_parse_set_model() {
    let model = BabyModel::new();
    assert!(modify::parse_set_model("   reg   accumulator   0xA   ", &model).is_ok());
    assert!(modify::parse_set_model("   mem   0xA   0xA   ", &model).is_ok());
    match modify::parse_set_model("   foo   0xA   0xA   ", &model) {
        Err(e) => assert!(e.starts_with("No such option.")),
        Ok(_) => panic!("Invalid args set model suceeded. ")
    }
}

#[test]
fn test_parse_set_config() {
    let conf = default_run();
    let test_logger = TestInterface::new_logger_test(format!("{}", modify::SET_HELP_MSG).as_str(), "", "");
    assert!(modify::parse_set_config("   debug-addrs   +   0xA   ", &conf, &test_logger).is_ok());
    assert!(modify::parse_set_config("   break-addrs   +   0xA   ", &conf, &test_logger).is_ok());
    assert!(modify::parse_set_config("   debug-regs   +   accumulator   ", &conf, &test_logger).is_ok());
    assert!(modify::parse_set_config("   help   ", &conf, &test_logger).is_ok());
    assert!(modify::parse_set_config("   ", &conf, &test_logger).is_ok());
    match modify::parse_set_config("   foo   barr   ", &conf, &test_logger) {
        Err(e) => assert!(e.starts_with("No recognised set command")),
        Ok(_) => panic!("Invalid args set config suceeded. ")
    }
}

#[test]
fn test_parse_set_command() {
    let model = BabyModel::new();
    let conf = default_run();
    let test_logger = TestInterface::new_logger_test(format!("{}", modify::SET_HELP_MSG).as_str(), "", "");

    assert!(modify::parse_set_command(format!("   reg   accumulator   0xA   "), &conf, &model, &test_logger).is_ok());
    assert!(modify::parse_set_command(format!("   debug-addrs   +   0xA   "), &conf, &model, &test_logger).is_ok());
    assert!(modify::parse_set_command(format!("   help   "), &conf, &model, &test_logger).is_ok());
    match modify::parse_set_command(format!("   sdfsdfdsfdsaf   "), &conf, &model, &test_logger) {
        Err(e) => e.starts_with("No such option as"),
        Ok(_) => panic!("Invalid argument to parse set command succeeded. ")
    };
}

#[test]
fn test_modify() {
    let wrong_command = format!("   sdfsdfdsfdsaf   ");
    let model = BabyModel::new();
    let conf = default_run();
    let test_logger = TestInterface::new_logger_test(
        format!("{}", modify::SET_HELP_MSG).as_str(), 
        format!("No such option as `{}`. ", wrong_command.clone().trim()).as_str(), 
        ""
    );

    modify::modify(format!("   help   "), &conf, &model, &test_logger);
    modify::modify(wrong_command, &conf, &model, &test_logger);
}

