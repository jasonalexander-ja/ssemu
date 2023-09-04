use crate::run::errors::RunError;
use crate::assemble::errors::AsmError;


pub enum Errors {
    RuntimeError(RunError),
    AsmError(AsmError)
}
