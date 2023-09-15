use std::path::PathBuf;
use baby_emulator::assembler::errors::AssemblyError;
use baby_emulator::core::MEMORY_WORDS;


/// Defines common behaviour for all errors thrown at runtime. 
pub trait RunError {
    fn describe(&self) -> String;
}

/// Possible errors thrown during reading the program source. 
pub enum SrcFileErrors {
    /// Failed to open a source file. 
    CouldntOpenFile(PathBuf),
    /// A binary file was of the wrong length. 
    BinFileWrongLen(usize),
    /// Failed to assemble a source file. 
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

/// All the possible error encountered at runtime. 
pub enum RunErrors {
    /// An error encountered getting the source file. 
    SrcFileError(SrcFileErrors)
}

impl RunError for RunErrors {
    fn describe(&self) -> String {
        match self {
            RunErrors::SrcFileError(s) => format!("{}", s.describe()),
        }
    }
}
