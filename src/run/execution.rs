use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
use baby_emulator::core::errors::BabyErrors;
use crate::args::Run;
use crate::interface::Interface;
use super::ProgramStack;
use super::debug::check_debug_session;

/// Iterates a model, returning the updated model plus any error encountered.  
/// 
/// # Parameters 
/// * `model` - The model to be interated. 
/// 
/// # Returns 
/// The updated model and any errors returned from the execution. 
/// 
fn iterate_model(model: &BabyModel) -> (BabyModel, Option<BabyErrors>) {
    match model.execute() {
        Ok(m) => (m, None),
        Err(e) => (model.clone(), Some(e))
    }
}

/// Determines if criteria has been met for running a debug session after a run. 
/// 
/// # Parameters 
/// * `model` - The model to check against. 
/// * `conf` - The configuration to check against. 
/// * `err_opt` - Possible errors returned from an execution. 
/// 
fn should_debug(model: &BabyModel, conf: &Run, err_opt: Option<BabyErrors>) -> bool {
    let has_hit_bp = conf.break_addr.contains(&(model.instruction_address as usize));
    let debug_on_err = conf.debug_on_err && err_opt.is_some();
    has_hit_bp || debug_on_err
}

/// Runs a model indefinitely until a stop command is encountered. 
/// 
/// # Parameters 
/// * `conf` - The configuration to run against. 
/// * `stack` - The program stack for the model to be initialised with. 
/// * `interface` - The interface used for interacting with the user. 
/// 
pub fn run_model(conf: Run, stack: ProgramStack, interface: &impl Interface) {
    let model = BabyModel::new_with_program(stack);
    let (mut model, mut conf) = (model.clone(), conf.clone());
    loop {
        let (new_model, err_opt) = iterate_model(&model);
        (model, conf) = if should_debug(&new_model, &conf, err_opt) {
            check_debug_session(&new_model, &conf, interface)
        } else { (model, conf) };

        if BabyInstruction::Stop == BabyInstruction::from_number(model.instruction) {
            break;
        }
    }
}
