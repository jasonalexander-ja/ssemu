use std::path::PathBuf;
use baby_emulator::assembler::errors::AssemblyError;


/// Defines common behaviour for errors thrown during assembling. 
pub trait AsmError {
    /// Gets a helper string describing an error. 
    fn describe(&self) -> String;
}

/// Possible errors when accessing a file. 
pub enum SrcFileErrors {
    /// Failed to read from a file. 
    CouldntOpenFile(PathBuf),
    /// Failed to write to a file. 
    CouldNotWriteToFile(PathBuf),
    /// Failed to serialise generated tag definitions. 
    FailedToSerialiseTags(serde_json::Error)
}

impl AsmError for SrcFileErrors {
    fn describe(&self) -> String {
        match self {
            SrcFileErrors::CouldntOpenFile(s) => 
                format!("Couldn't open asm source file: `{}`.", s.to_string_lossy().to_string()),
            SrcFileErrors::CouldNotWriteToFile(s) => 
                format!("Couldn't write assembled data to file: `{}`.", s.to_string_lossy().to_string()),
            SrcFileErrors::FailedToSerialiseTags(s) => 
                format!("Failed to serialise the generated tag definitions with:  `{}`.", s.to_string()),
        }
    }
}

/// Possible errors thrown during assembling a source file. 
pub enum AsmErrors {
    /// Error accessing a file. 
    SrcFileError(SrcFileErrors),
    /// Error assembling. 
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
