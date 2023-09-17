use super::output;
use crate::test_utils::{TestInterface, TestSucessiveInterface};
use crate::args::Registers;
use baby_emulator::core::{BabyModel, MEMORY_WORDS, instructions::BabyInstruction};


#[test]
fn test_output_model_all() {
    let model = BabyModel::new();
    let test_int = TestInterface::new_logger_test(model.core_dump().as_str(), "", "");
    output::output_model(&vec![], &vec![], true, &model, &test_int);
}

#[test]
fn test_output_model_addresses() {
    let model = BabyModel::new();
    let test_int = TestInterface::new_logger_test(
        format!("{:#04x}: {:#010x}", 0, model.main_store[0 & 0x1F]).as_str(), "", ""
    );
    output::output_model(&vec![], &vec![0], false, &model, &test_int);
}

#[test]
fn test_output_register() {
    let model = BabyModel::new();
    let test_int = TestInterface::new_logger_test(format!("{:#010x}", model.accumulator).as_str(), "", "");
    output::output_register(&Registers::Accumulator, &model, &test_int);
    
    let test_int = TestInterface::new_logger_test(format!("{:#010x}", model.instruction_address).as_str(), "", "");
    output::output_register(&Registers::InstructionAddress, &model, &test_int);
    
    let test_int = TestInterface::new_logger_test(format!(
        "{:#010x} ({})", 
        model.instruction, 
        BabyInstruction::from_number(model.instruction).get_instr_description()
    ).as_str(), "", "");
    output::output_register(&Registers::Instruction, &model, &test_int);
}

#[test]
fn test_output_all_registers() {
    let model = BabyModel::new();
    let test_int = TestSucessiveInterface::new_logger_test(
        vec![
            format!("{:#010x}", model.accumulator).as_str(),
            format!(
                "{:#010x} ({})", 
                model.instruction, 
                BabyInstruction::from_number(model.instruction).get_instr_description()
            ).as_str(),
            format!("{:#010x}", model.instruction_address).as_str()
        ], 
        vec![], 
        vec![]
    );
    output::output_all_registers(&model, &test_int);
}

#[test]
fn test_output_all_memory() {
    let model = BabyModel::new();
    let mut should_log: Vec<String> = Vec::new();
    let mut test_int = TestSucessiveInterface::new_logger_test(vec![], vec![], vec![]);
    for v in 0..MEMORY_WORDS {
        should_log.push(format!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F]));
    }
    test_int.should_log = should_log;
    output::output_all_memory(&model, &test_int);
}
