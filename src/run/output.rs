use baby_emulator::core::{MEMORY_WORDS, BabyModel, instructions::BabyInstruction};
use strum::IntoEnumIterator;
use crate::args::Registers;
use crate::interface::Interface;



/// Outputs a register of a model, formatted, to an interface. 
/// 
/// # Parameters 
/// * `regs` - The register to be outputted. 
/// * `model` - The model to be read. 
/// * `int` - The interface to output to. 
///  
fn output_register(regs: &Registers, model: &BabyModel, int: &impl Interface) {
    match regs {
        Registers::Accumulator => int.log_msg(format!("{:#010x}", model.accumulator)),
        Registers::InstructionAddress => int.log_msg(format!("{:#010x}", model.instruction_address)),
        Registers::Instruction => 
            int.log_msg(format!(
                "{:#010x} ({})", 
                model.instruction, 
                BabyInstruction::from_number(model.instruction).get_instr_description()
            )),
    }
}

/// Outputs the all the registers of a model, formatted to an interface. 
/// 
/// # Parameters 
/// * `model` - The model to be read. 
/// * `int` - The interface to output to. 
///  
pub fn output_all_registers(model: &BabyModel, int: &impl Interface) {
    for v in Registers::iter() {
        output_register(&v, model, int);
    }
}

/// Outputs the entire memory of a model, formatted to an interface. 
/// 
/// # Parameters 
/// * `model` - The model to be read. 
/// * `int` - The interface to output to. 
///  
pub fn output_all_memory(model: &BabyModel, int: &impl Interface) {
    for v in 0..MEMORY_WORDS {
        int.log_msg(format!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F]));
    }
}

/// Outputs a model based on the configuration, formatted to an interface. 
/// 
/// # Parameters 
/// * `registers` - Any registers to be outputted. 
/// * `memory_addrs` - Any memory addresses to be outputted. 
/// * `output_model` - Output everything in the model. 
/// * `model` - The model to be read. 
/// * `int` - The interface to be outputted to. 
/// 
pub fn output_model(
    registers: &Vec<Registers>, 
    memory_addrs: &Vec<usize>, 
    output_model: bool, 
    model: &BabyModel,
    int: &impl Interface
) {
    if output_model {
        int.log_msg(model.core_dump());
        return;
    }
    registers.iter().for_each(|v| output_register(v, &model, int));
    memory_addrs.iter().for_each(|v| 
        int.log_msg(format!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F]))
    );
}
