use std::path::PathBuf;
use baby_emulator::assembler::errors::AssemblyError;


pub trait AsmError {
    fn describe(&self) -> String;
}

pub enum SrcFileErrors {
    CouldntOpenFile(PathBuf),
    CouldNotWriteToFile(PathBuf),
}

impl AsmError for SrcFileErrors {
    fn describe(&self) -> String {
        match self {
            SrcFileErrors::CouldntOpenFile(s) => 
                format!("Couldn't open asm source file: `{}`.", s.to_string_lossy().to_string()),
            SrcFileErrors::CouldNotWriteToFile(s) => 
                format!("Couldn't write assembled data to file: `{}`.", s.to_string_lossy().to_string()),
        }
    }
}

pub enum AsmErrors {
    SrcFileError(SrcFileErrors),
    AssembleError(AssemblyError)
}

impl AsmError for AsmErrors {
    fn describe(&self) -> String {
        match self {
            AsmErrors::AssembleError(s) => format!("Problem assembling file. \n{}", s.describe(true)),
            AsmErrors::SrcFileError(s) => format!("Issue accessing a file. \n{}", s.describe()),
        }
    }
}
