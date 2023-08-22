use baby_emulator::core::BabyModel;
use baby_emulator::core::errors::BabyErrors;
use colored::Colorize;
use crate::args::Run;


pub fn check_debug_session(model: &BabyModel, conf: &Run, err: Option<BabyErrors>) {
    loop {
        println!("foo".cyan());
    }
}
