use std::path::PathBuf;
use baby_emulator::assembler::assemble as asm;
use baby_emulator::core::instructions::BabyInstruction;
use crate::args::Assemble;
use crate::errors::Errors;
use crate::interface::Interface;
use crate::run::ProgramStack;
use errors::{AsmErrors, SrcFileErrors};

/// Possible error thrown during assembling. 
pub mod errors;


/// Reads a source asm from an interface and assembles it into a program stack. 
/// 
/// # Parameters 
/// * `source` - The source path to the asm file. 
/// * `og_notation` - Whether to use original notation for the assembling. 
/// * `interface` - The interface to read from. 
/// 
/// # Returns 
/// * [Ok(ProgramStack)] - The assembles program stack. 
/// * [Err(AsmErrors)] - There was an error reading the asm source or assembling. 
/// 
fn get_src_from_asm(
    source: &PathBuf, 
    og_notation: bool,
    interface: &impl Interface
) -> Result<ProgramStack, AsmErrors> {
    let a = interface.read_fs_string(source)
        .map_err(|_| AsmErrors::SrcFileError(SrcFileErrors::CouldntOpenFile(source.clone())))?;

    let res = asm(&a, og_notation)
        .map_err(|e| AsmErrors::AssembleError(e))?;

    Ok(BabyInstruction::to_numbers(res))
}

/// Formats the program stack as bytes and writes it to an interface. 
/// 
/// # Parameters 
/// * `data` - The data to be formatted and written. 
/// * `conf` - The configration to be used. 
/// * `interface` - The interface to write to. 
/// 
/// # Returns 
/// * [Ok(())] - Writing happened successfully. 
/// * [Err(AsmErrors)] - Error encountered during writing. 
/// 
fn write_to_file(
    data: ProgramStack, 
    conf: &Assemble,
    interface: &impl Interface
) -> Result<(), AsmErrors> {
    let out = match &conf.output {
        Some(v) => v.clone(),
        None => PathBuf::from(conf.input.to_string_lossy().to_string() + ".bin")
    };

    let d: Vec<u8> = data.iter().flat_map(|v| {
        (0..4).map(|i| (v >> ((3 - i) * 8)) as u8).collect::<Vec<u8>>()
    }).collect();

    interface.write_fs_bytes(d, &out)
        .map_err(|_| AsmErrors::SrcFileError(SrcFileErrors::CouldNotWriteToFile(out.clone())))?;

    Ok(())
}

/// Attempts to read an asm string from an interface, assemble it, and write it back
/// to an interface. 
/// 
/// # Parameters 
/// * `conf` - The configuration to be used. 
/// * `interface` - The interface to be used for writing/reading. 
/// 
/// # Returns 
/// * [Ok(())] - Assembling and writing happened sucessfully. 
/// * [Err(Errors)] - An error was encountered during assembling/writing. 
/// 
pub fn assemble(conf: Assemble, interface: &impl Interface) -> Result<(), Errors> {
    let bin = get_src_from_asm(&conf.input, conf.og_notation, interface)
        .map_err(|e| Errors::AsmError(e))?;

    write_to_file(bin, &conf, interface)
        .map_err(|e| Errors::AsmError(e))?;

    Ok(())
}
