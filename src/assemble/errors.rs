use std::path::PathBuf;
use baby_emulator::assembler::errors::AssemblyError;


pub enum SrcFileError {
    CouldntOpenFile(PathBuf),
    CouldNotWriteToFile(PathBuf),
}

pub enum AsmError {
    SrcFileError(SrcFileError),
    AssembleError(AssemblyError)
}
