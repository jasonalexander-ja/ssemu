use baby_emulator::core::{MEMORY_WORDS, BabyModel, instructions::BabyInstruction};
use strum::IntoEnumIterator;
use crate::args::Registers;
use crate::interface::Interface;


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

pub fn output_all_registers(model: &BabyModel, int: &impl Interface) {
    for v in Registers::iter() {
        output_register(&v, model, int);
    }
}

pub fn output_all_memory(model: &BabyModel, int: &impl Interface) {
    for v in 0..MEMORY_WORDS {
        int.log_msg(format!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F]));
    }
}

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
