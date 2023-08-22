use std::path::PathBuf;
use baby_emulator::assembler::errors::AssemblyError;


pub enum SrcFileError {
    CouldntOpenFile(PathBuf),
    BinFileWrongLen(usize),
    AssembleError(AssemblyError)
}

pub enum RunError {
    SrcFileError(SrcFileError)
}
