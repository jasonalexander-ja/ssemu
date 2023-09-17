use baby_emulator::core::BabyModel;
use baby_emulator::core::instructions::BabyInstruction;
use crate::args::Run;
use crate::interface::Interface;
use super::print::print;
use super::modify::modify;


/// A help messages for a list of debug commands. 
pub const HELP: &str = 
"Possible commands:
print - Print the value of a register or memory location(s). 
set - Set a memory location, register, or set a breakpoint, or memorylocation/register to print on debug. 
end - End execution. 
help - Print this help command
";


/// Finds a matching debug command and dispatches the relevant actions. 
/// 
/// Returns the model and configuration containing any changes. 
/// 
/// # Parameters 
/// * `command` - The string command stating what is being set. 
/// * `model` - The model to be acted upon. 
/// * `conf` - The configuration model to be acted upon. 
/// * `int` - The interface to print messages. 
/// 
pub fn match_debug_command(
    command: String, 
    conf: &Run, 
    model: &BabyModel, 
    int: &impl Interface
) -> (BabyModel, Run) {
    let command = command.trim().to_lowercase();
    let (next_com, _) = command.split_at(command.find(" ").unwrap_or(command.len()));
    let next_com = next_com.trim();
    match next_com {
        "s" | "set" => modify(command.replace("set", ""), conf, model, int),
        "p" | "print" => {
            print(command.replace("print", ""), conf, model, int);
            (model.clone(), conf.clone())
        },

        "h" | "" | "help" => {
            int.log_msg(format!("{}", HELP));
            (model.clone(), conf.clone())
        },

        "e" | "end" => {
            let mut model = model.clone();
            model.instruction = BabyInstruction::Stop.to_number() as u16;
            (model, conf.clone())
        },
        
        _ => {
            int.log_warn(format!("No such command as `{}`, use help for a list of commands. ", next_com));
            (model.clone(), conf.clone())
        }
    }
}
