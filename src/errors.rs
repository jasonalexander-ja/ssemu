use super::run::errors::RunError;
use super::assemble::errors::AssembleError;


pub enum Errors {
    RunError(RunError),
    AssembleError(AssembleError)
}
