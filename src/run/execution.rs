use baby_emulator::core::BabyModel;
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

pub fn run_model(conf: Run, stack: ProgramStack) {
    let model = BabyModel::new_with_program(stack);
    loop {
        let (model, err_opt) = iterate_model(&model);
        let has_hit_bp = conf.break_addr.contains(&(model.instruction_address as usize));
        let debug_on_err = conf.debug_on_err && err_opt.is_some();
        if has_hit_bp || debug_on_err {
            check_debug_session(&model, &conf);
        }
    }
}
