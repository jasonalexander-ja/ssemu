use std::path::PathBuf;
use baby_emulator::core::{MEMORY_WORDS, instructions::BabyInstruction};
use baby_emulator::assembler::assemble;
use super::ProgramStack;
use super::errors::{SrcFileErrors, RunErrors};
use crate::args::{Run, ExecuteFrom};
use crate::interface::Interface;


/// Reads a file from the interface and attempts to assemble it into a program stack. 
/// 
/// # Parameters 
/// * `source` - The source asm file. 
/// * `og_notation` - Whether to use original asm notation for assembling. 
/// * `interface` - The interface to be used to read the asm source file. 
/// 
/// # Returns 
/// * [Ok(ProgramStack)] - The successfully assembled program. 
/// * [Err(SrcFileErrors)] - An error details. 
/// 
fn get_src_from_asm(
    source: &PathBuf, 
    og_notation: bool, 
    interface: &impl Interface
) -> Result<ProgramStack, SrcFileErrors> {
    let asm = interface.read_fs_string(source)
        .map_err(|_| SrcFileErrors::CouldntOpenFile(source.clone()))?;

    let res = assemble(&asm, og_notation)
        .map_err(|e| SrcFileErrors::AssembleError(e))?;

    Ok(BabyInstruction::to_numbers(res))
}

/// Provides a function to read an i32 word from 4 bytes in an array. 
/// 
/// Takes an array of the bytes, returning a closure that accepts 
/// the index of the word and returns the generated word. 
/// 
pub fn read_word<'a>(raw: &'a Vec<u8>) -> impl Fn(usize) -> i32 + 'a {
    |i: usize| 
        (0..4).fold(0, |val, j| val + (*raw.get(i + j).unwrap_or(&0) as i32) << ((3 - j) * 8))
}

/// Reads groups of 4 bytes into i32 words. 
/// 
/// # Parameters 
/// * `raw` - The raw bytes. 
/// 
pub fn read_words(raw: Vec<u8>) -> [i32; MEMORY_WORDS] {
    core::array::from_fn(read_word(&raw))
}

/// Reads a binary file from the interface and attempts to fit it into a program stack. 
/// 
/// # Parameters 
/// * `source` - The binary file source. 
/// * `interface` - The interface to be used to read the binary source file. 
/// 
/// # Returns 
/// * [Ok(ProgramStack)] - The successfully read program. 
/// * [Err(SrcFileErrors)] - An error details. 
/// 
fn from_bin(source: &PathBuf, interface: &impl Interface) -> Result<ProgramStack, SrcFileErrors> {
    let raw = interface.read_fs_bytes(source)
        .map_err(|_| SrcFileErrors::CouldntOpenFile(source.clone()))?;

    if raw.len() != MEMORY_WORDS * 4 {
        return Err(SrcFileErrors::BinFileWrongLen(raw.len()))
    }

    let res = read_words(raw);

    Ok(res)
}

/// Attempts to get the program source based on the configuration and 
/// a given file system interface. 
/// 
/// # Parameters 
/// * `config` - The configuration to use. 
/// * `interface` - The interface to use. 
/// 
pub fn get_src(config: &Run, interface: &impl Interface) -> Result<ProgramStack, RunErrors> {
    match config.exe_from {
        ExecuteFrom::Asm => get_src_from_asm(&config.src, config.og_notation, interface)
            .map_err(|e| RunErrors::SrcFileError(e)),
        ExecuteFrom::Bin => from_bin(&config.src, interface)
            .map_err(|e| RunErrors::SrcFileError(e)),
    }
}
