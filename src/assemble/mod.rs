use std::path::PathBuf;
use std::fs;
use baby_emulator::assembler::assemble as asm;
use baby_emulator::core::instructions::BabyInstruction;
use baby_emulator::core::MEMORY_WORDS;
use crate::args::Assemble;
use crate::errors::Errors;
use errors::{AsmError, SrcFileError};

pub mod errors;


fn get_src_from_asm(source: &PathBuf, og_notation: bool) -> Result<[i32; MEMORY_WORDS], AsmError> {
    let a = fs::read_to_string(source)
        .map_err(|_| AsmError::SrcFileError(SrcFileError::CouldntOpenFile(source.clone())))?;

    let res = asm(&a, og_notation)
        .map_err(|e| AsmError::AssembleError(e))?;

    Ok(BabyInstruction::to_numbers(res))
}

fn write_to_file(data: [i32; MEMORY_WORDS], conf: &Assemble) -> Result<(), AsmError> {
    let out = match &conf.output {
        Some(v) => v.clone(),
        None => PathBuf::from(conf.input.to_string_lossy().to_string() + ".bin")
    };

    let d: Vec<u8> = data.iter().flat_map(|v| {
        (0..4).map(|i| (v >> ((3 - i) * 8)) as u8).collect::<Vec<u8>>()
    }).collect();

    fs::write(&out, &d)
        .map_err(|_| AsmError::SrcFileError(SrcFileError::CouldNotWriteToFile(out.clone())))?;

    Ok(())
}

pub fn assemble(conf: Assemble) -> Result<(), Errors> {
    let bin = get_src_from_asm(&conf.input, conf.og_notation)
        .map_err(|e| Errors::AsmError(e))?;

    write_to_file(bin, &conf)
        .map_err(|e| Errors::AsmError(e))?;

    Ok(())
}
