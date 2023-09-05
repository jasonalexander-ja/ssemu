use crate::run::errors::{RunErrors, RunError};
use crate::assemble::errors::{AsmErrors, AsmError};


pub enum Errors {
    RuntimeError(RunErrors),
    AsmError(AsmErrors)
}

impl Errors {
    pub fn describe(&self) -> String {
        match self {
            Errors::AsmError(v) => format!("{}", v.describe()),
            Errors::RuntimeError(v) => format!("{}", v.describe()),
        }
    }
}
