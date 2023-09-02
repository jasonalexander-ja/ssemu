use baby_emulator::core::BabyModel;
use baby_emulator::core::instructions::BabyInstruction;
use crate::args::Run;
use super::print_debug::print;
use super::modify::modify;


const HELP: &str = 
"Possible commands:
print - Print the value of a register or memory location(s)  
help - Print this help command
";

pub fn match_debug_command(command: String, conf: &Run, model: &BabyModel) -> (BabyModel, Run) {
    let command = command.trim();
    match command.split(" ").next() {
        Some("set") => modify(command.replace("set", ""), conf, model),
        Some("print") => {
            print(command.replace("print", ""), conf, model);
            (model.clone(), conf.clone())
        },

        Some("") | Some("help") => {
            println!("{}", HELP);
            (model.clone(), conf.clone())
        },

        Some("end") => {
            let mut model = model.clone();
            model.instruction = BabyInstruction::Stop.to_number() as u16;
            (model, conf.clone())
        },
        
        _ => (model.clone(), conf.clone())
    }
}
