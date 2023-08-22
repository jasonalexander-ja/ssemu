use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
use crate::args::{Run, Registers};


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

pub fn output_model(config: Run, model: BabyModel) {
    if config.output_model {
        println!("{}", model.core_dump());
        return;
    }
    config.output_regs.iter().for_each(|v| output_register(v, &model));
    config.output_addr.iter().for_each(|v| 
        println!("{:#04x}: {:#010x}", v, model.main_store[v & 0x1F])
    );
}
