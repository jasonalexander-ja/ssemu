use baby_emulator::core::{BabyModel, instructions::BabyInstruction};
use baby_emulator::core::errors::BabyErrors;
use crate::args::Run;
use super::ProgramStack;
use super::debug::check_debug_session;

fn iterate_model(model: &BabyModel) -> (BabyModel, Option<BabyErrors>) {
    match model.execute() {
        Ok(m) => (m, None),
        Err(e) => (model.clone(), Some(e))
    }
}

fn should_debug(model: &BabyModel, conf: &Run, err_opt: Option<BabyErrors>) -> bool {
    let has_hit_bp = conf.break_addr.contains(&(model.instruction_address as usize));
    let debug_on_err = conf.debug_on_err && err_opt.is_some();
    has_hit_bp || debug_on_err
}

pub fn run_model(conf: Run, stack: ProgramStack) {
    let model = BabyModel::new_with_program(stack);
    let (mut model, mut conf) = (model.clone(), conf.clone());
    loop {
        let (new_model, err_opt) = iterate_model(&model);
        (model, conf) = if should_debug(&new_model, &conf, err_opt) {
            check_debug_session(&new_model, &conf)
        } else { (model, conf) };

        if BabyInstruction::Stop == BabyInstruction::from_number(model.instruction) {
            break;
        }
    }
}
