use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use baby_emulator::core::{BabyModel, MEMORY_WORDS, instructions::BabyInstruction};
use crate::test_utils::{TestInterface, TestSucessiveInterface, TestApplyInterface};
use crate::args::{Run, Registers, ExecuteFrom};
use super::print;
use super::print::PRINT_HELP;


#[test]
fn test_show_registers() {
    let model = BabyModel::new();
    let test_err_int = TestInterface::new_logger_test("", "Invalid register name: foo", "");
    let mut test_int = TestSucessiveInterface::new_logger_test(
        vec![
            format!("{:#010x}", model.accumulator).as_str(),
            format!(
                "{:#010x} ({})", 
                model.instruction, 
                BabyInstruction::from_number(model.instruction).get_instr_description()
            ).as_str(),
            format!("{:#010x}", model.instruction_address).as_str(),
        ], 
        vec![], 
        vec![]
    );
    print::show_registers(format!("   Accumulator   ,   instruction   ,   InstructionAddress   "), &model, &test_int);
    test_int.log_count = AtomicUsize::new(0);
    print::show_registers(format!("      "), &model, &test_int);
    print::show_registers(format!("   Accumulator   ,   foo   "), &model, &test_err_int);
}

#[test]
fn test_show_memory_addresses() {
    let model = BabyModel::new();
    let apply_int = TestApplyInterface::new_logger_test(
        |v| assert!(v.starts_with("Invalid memory address:")), 
        |_| {}, 
        |_| {}
    );
    let mut should_log: Vec<String> = Vec::new();
    for v in 0..MEMORY_WORDS {
        should_log.push(format!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F]));
    }
    let mut test_int = TestSucessiveInterface::new_logger_test(
        vec![
            format!("{:#04x}: {:#010x}", 0, model.main_store[0 & 0x1F]).as_str(),
            format!("{:#04x}: {:#010x}", 1, model.main_store[1 & 0x1F]).as_str(),
            format!("{:#04x}: {:#010x}", 2, model.main_store[2 & 0x1F]).as_str(),
            format!("{:#04x}: {:#010x}", 10, model.main_store[10 & 0x1F]).as_str(),
        ], 
        vec![], 
        vec![]
    );
    print::show_memory_addresses(format!("   0   ,   0b1   ,   0o2   ,   0xA   "), &model, &test_int);
    test_int.log_count = AtomicUsize::new(0);
    test_int.should_log = should_log;
    print::show_memory_addresses(format!("      "), &model, &test_int);
    print::show_memory_addresses(format!("   0   ,   0xY   "), &model, &apply_int);
}

#[test]
fn test_print_addresses() {
    let addrs = vec![0, 1, 2, 3];
    let mut test_int = TestInterface::new_logger_test("", "", "");
    test_int.should_log = addrs.iter()
        .map(|v| format!("{:#04x}", v))
        .collect::<Vec<String>>()
        .join(", ");
    print::print_addresses(&addrs, &test_int);
}

#[test]
fn test_print_registers() {
    let regs = vec![Registers::Accumulator, Registers::Instruction, Registers::InstructionAddress];
    let mut test_int = TestInterface::new_logger_test("", "", "");
    test_int.should_log = regs.iter()
        .map(|v| format!("{:?}", v))
        .collect::<Vec<String>>()
        .join(", ");
    print::print_registers(&regs, &test_int);
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
fn test_print_help() {
    let conf = default_run();
    let model = BabyModel::new();
    let test_int = TestInterface::new_logger_test(
        format!("{}", PRINT_HELP).as_str(), 
        "", 
        ""
    );
    print::print(format!("   help   "), &conf, &model, &test_int);
    print::print(format!("   h   "), &conf, &model, &test_int);
}

#[test]
fn test_print_err() {
    let conf = default_run();
    let model = BabyModel::new();
    let test_int = TestApplyInterface::new_logger_test(
        |_| {}, 
        |s| assert!(s.starts_with("No recognised print command")), 
        |_| {}
    );
    print::print(format!("   foo   "), &conf, &model, &test_int);
}
