use crate::run::errors::{RunErrors, RunError};
use crate::assemble::errors::{AsmErrors, AsmError};


/// All the possible errors thrown during execution of the application. 
pub enum Errors {
    /// An error encountered executing the emulation. 
    RuntimeError(RunErrors),
    /// An error encountered assembling an asm source file. 
    AsmError(AsmErrors)
}

impl Errors {
    /// Gets a helper string describing an error. 
    pub fn describe(&self) -> String {
        match self {
            Errors::AsmError(v) => format!("{}", v.describe()),
            Errors::RuntimeError(v) => format!("{}", v.describe()),
        }
    }
}
