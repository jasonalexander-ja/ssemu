use std::path::PathBuf;
use super::commands;
use super::commands::HELP;
use crate::test_utils::TestInterface;
use crate::args::{Run, ExecuteFrom, Registers};
use baby_emulator::core::{BabyModel, instructions::BabyInstruction};


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
fn test_match_debug_command_help() {
    let model = BabyModel::new();
    let conf = default_run();
    let test_int = TestInterface::new_logger_test(format!("{}", HELP).as_str(), "", "");
    commands::match_debug_command(format!("      "), &conf, &model, &test_int);
    commands::match_debug_command(format!("   h   "), &conf, &model, &test_int);
    commands::match_debug_command(format!("   H   "), &conf, &model, &test_int);
    commands::match_debug_command(format!("   HELP   "), &conf, &model, &test_int); 
    commands::match_debug_command(format!("   help   "), &conf, &model, &test_int); 
}

#[test]
fn test_match_debug_command_end() {
    let model = BabyModel::new();
    let conf = default_run();
    let test_int = TestInterface::new_logger_test(format!("{}", HELP).as_str(), "", "");

    let (m, _) = commands::match_debug_command(format!("   end   "), &conf, &model, &test_int);
    assert_eq!(m.instruction, BabyInstruction::Stop.to_number() as u16);

    let (m, _) = commands::match_debug_command(format!("   END   "), &conf, &model, &test_int);
    assert_eq!(m.instruction, BabyInstruction::Stop.to_number() as u16);

    let (m, _) = commands::match_debug_command(format!("   e   "), &conf, &model, &test_int);
    assert_eq!(m.instruction, BabyInstruction::Stop.to_number() as u16);

    let (m, _) = commands::match_debug_command(format!("   E   "), &conf, &model, &test_int);
    assert_eq!(m.instruction, BabyInstruction::Stop.to_number() as u16);
}

#[test]
fn test_match_debug_command_err() {
    let model = BabyModel::new();
    let conf = default_run();
    let test_int = TestInterface::new_logger_test("", "No such command as `foo`, use help for a list of commands. ", "");
    commands::match_debug_command(format!("   foo   "), &conf, &model, &test_int);
}

