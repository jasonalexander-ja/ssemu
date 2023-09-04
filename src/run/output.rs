use baby_emulator::core::{MEMORY_WORDS, BabyModel, instructions::BabyInstruction};
use strum::IntoEnumIterator;
use crate::args::Registers;


fn output_register(regs: &Registers, model: &BabyModel) {
    match regs {
        Registers::Accumulator => println!("{:#010x}", model.accumulator),
        Registers::InstructionAddress => println!("{:#010x}", model.instruction_address),
        Registers::Instruction => 
            println!(
                "{:#010x} ({})", 
                model.instruction, 
                BabyInstruction::from_number(model.instruction).get_instr_description()
            ),
    }
}

pub fn output_all_registers(model: &BabyModel) {
    for v in Registers::iter() {
        output_register(&v, model);
    }
}

pub fn output_all_memory(model: &BabyModel) {
    for v in 0..MEMORY_WORDS {
        println!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F]);
    }
}

pub fn output_model(registers: &Vec<Registers>, memory_addrs: &Vec<usize>, output_model: bool, model: &BabyModel) {
    if output_model {
        println!("{}", model.core_dump());
        return;
    }
    registers.iter().for_each(|v| output_register(v, &model));
    memory_addrs.iter().for_each(|v| 
        println!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F])
    );
}
