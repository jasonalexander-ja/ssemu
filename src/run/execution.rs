use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
use baby_emulator::core::errors::BabyErrors;
use crate::args::Run;
use crate::interface::Interface;
use super::ProgramStack;
use super::debug::{check_debug_session, DebugResult};
use super::output::output_model;

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
fn should_debug(model: &BabyModel, conf: &Run, err_opt: &Option<BabyErrors>) -> bool {
    let has_hit_bp = conf.break_addr.contains(&(model.instruction_address as usize));
    let debug_on_err = conf.debug_on_err && err_opt.is_some();
    has_hit_bp || debug_on_err
}

fn check_run_debug(
    debug_next: Option<()>,
    model: BabyModel, 
    conf: Run, 
    err_opt: &Option<BabyErrors>, 
    int: &impl Interface
) -> DebugResult {
    if should_debug(&model, &conf, err_opt) || debug_next.is_some() {
        check_debug_session(&model, &conf, int)
    } else { DebugResult::Continue(model, conf) }
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
    let mut debug_next: Option<()> = None;
    let mut err_opt: Option<BabyErrors> = None;
    loop {
        let debug_res = check_run_debug(debug_next, model, conf, &err_opt, interface);
        
        (model, conf) = match debug_res {
            DebugResult::Continue(m, c) => { debug_next = None; (m, c) },
            DebugResult::SingleStep(m, c) => { debug_next = Some(()); (m, c) },
            DebugResult::End(m, c) => {
                (model, conf) = (m, c);
                break;
            }
        };
        
        (model, err_opt) = iterate_model(&model);

        if BabyInstruction::Stop == BabyInstruction::from_number(model.instruction) {
            break;
        }
    }
    output_model(&conf.output_regs, &conf.output_addr, conf.output_model, &model, interface);
}
