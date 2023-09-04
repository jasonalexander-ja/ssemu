use std::path::PathBuf;
use baby_emulator::assembler::errors::AssemblyError;
use baby_emulator::core::MEMORY_WORDS;


pub trait RunError {
    fn describe(&self) -> String;
}

pub enum SrcFileErrors {
    CouldntOpenFile(PathBuf),
    BinFileWrongLen(usize),
    AssembleError(AssemblyError)
}

impl RunError for SrcFileErrors {
    fn describe(&self) -> String {
        match self {
            SrcFileErrors::CouldntOpenFile(s) => 
                format!("Could not open source asm/binary file: `{}`.", s.to_string_lossy().to_string()),
            SrcFileErrors::BinFileWrongLen(s) => 
                format!("The baby memory is {} words long, specified file is `{}` words long. ", MEMORY_WORDS, s/4),
            SrcFileErrors::AssembleError(s) => 
                format!("Error assembling source asm file. \n{}", s.describe(true)),
        }
    }
}

pub enum RunErrors {
    SrcFileError(SrcFileErrors)
}

impl RunError for RunErrors {
    fn describe(&self) -> String {
        match self {
            RunErrors::SrcFileError(s) => format!("{}", s.describe()),
        }
    }
}
