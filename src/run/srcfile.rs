use std::path::PathBuf;
use std::fs;
use baby_emulator::core::{MEMORY_WORDS, instructions::BabyInstruction};
use baby_emulator::assembler::assemble;
use super::ProgramStack;
use super::errors::{SrcFileErrors, RunErrors};
use crate::args::{Run, ExecuteFrom};


fn get_src_from_asm(source: &PathBuf, og_notation: bool) -> Result<ProgramStack, SrcFileErrors> {
    let asm = fs::read_to_string(source)
        .map_err(|_| SrcFileErrors::CouldntOpenFile(source.clone()))?;

    let res = assemble(&asm, og_notation)
        .map_err(|e| SrcFileErrors::AssembleError(e))?;

    Ok(BabyInstruction::to_numbers(res))
}

fn from_bin(source: &PathBuf) -> Result<ProgramStack, SrcFileErrors> {
    let raw = fs::read(source)
        .map_err(|_| SrcFileErrors::CouldntOpenFile(source.clone()))?;

    if raw.len() != MEMORY_WORDS * 4 {
        return Err(SrcFileErrors::BinFileWrongLen(raw.len()))
    }

    let res: [i32; MEMORY_WORDS] = core::array::from_fn(|i| 
        (0..4).fold(0, |val, j| val + (raw[i + j] as i32) << ((3 - j) * 8))
    );

    Ok(res)
}

pub fn get_src(config: &Run) -> Result<ProgramStack, RunErrors> {
    match config.exe_from {
        ExecuteFrom::Asm => get_src_from_asm(&config.src, config.og_notation)
            .map_err(|e| RunErrors::SrcFileError(e)),
        ExecuteFrom::Bin => from_bin(&config.src)
            .map_err(|e| RunErrors::SrcFileError(e)),
    }
}
