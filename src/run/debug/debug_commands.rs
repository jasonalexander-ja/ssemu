use baby_emulator::core::BabyModel;
use baby_emulator::core::instructions::BabyInstruction;
use crate::args::Run;
use crate::interface::Interface;
use super::print_debug::print;
use super::modify::modify;


const HELP: &str = 
"Possible commands:
print - Print the value of a register or memory location(s). 
set - Set a memory location, register, or set a breakpoint, or memorylocation/register to print on debug. 
end - End execution. 
help - Print this help command
";

pub fn match_debug_command(
    command: String, 
    conf: &Run, 
    model: &BabyModel, 
    int: &impl Interface
) -> (BabyModel, Run) {
    let command = command.trim();
    match command.split(" ").next() {
        Some("s") | Some("set") => modify(command.replace("set", ""), conf, model, int),
        Some("p") | Some("print") => {
            print(command.replace("print", ""), conf, model, int);
            (model.clone(), conf.clone())
        },

        Some("h") | Some("") | Some("help") => {
            int.log_msg(format!("{}", HELP));
            (model.clone(), conf.clone())
        },

        Some("e") | Some("end") => {
            let mut model = model.clone();
            model.instruction = BabyInstruction::Stop.to_number() as u16;
            (model, conf.clone())
        },
        
        _ => (model.clone(), conf.clone())
    }
}
