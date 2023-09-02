use std::path::PathBuf;
use std::num::ParseIntError;
use clap::{Parser, Subcommand, Args, ValueEnum};
use baby_emulator::core::MEMORY_WORDS;
use strum_macros::EnumIter;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Assemble an asm source file to a binary file. 
    Assemble(Assemble),
    /// Load and run a source file. 
    Run(Run)
}

#[derive(Args)]
pub struct Assemble {
    /// The input asm file. 
    #[arg(long, short)]
    pub input: PathBuf,

    /// The output binary dump. 
    #[arg(long, short)]
    pub output: Option<PathBuf>,

    /// Use original notation for asm instructions. 
    #[arg(long, default_value_t = false)]
    pub og_notation: bool,
}

#[derive(Args, Clone)]
pub struct Run {
    /// The source file to execute from. 
    pub src: PathBuf,

    /// The format of the file to execute from. 
    #[arg(long, default_value_t = ExecuteFrom::Bin, value_enum)]
    pub exe_from: ExecuteFrom,

    /// Use original notation for asm instructions if running from asm. 
    #[arg(long, default_value_t = false)]
    pub og_notation: bool,
    
    /// Output whole `model` including registers & memory when execution stops or breakpoint encountered. 
    #[arg(long, default_value_t = false)]
    pub output_model: bool,
    
    /// Go into a debug session when an error is encountered, as opposed to exiting. 
    #[arg(long, default_value_t = false)]
    pub debug_on_err: bool,

    /// Memory addresses to output when execution stops or breakpoint encountered. 
    #[arg(long, value_parser = parse_output_addresses)]
    pub output_addr: Vec<usize>,

    /// Registers to output when execution stops or breakpoint encountered. 
    #[arg(long, value_enum)]
    pub output_regs: Vec<Registers>,

    /// Addresses where to break & output the state of the core. 
    #[arg(long, value_parser = parse_breakpoint_addresses)]
    pub break_addr: Vec<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExecuteFrom {
    /// Executes from an assembly file. 
    Asm,
    /// Executes from a binary dump. 
    Bin,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter)]
pub enum Registers {
    /// The accumulator register. 
    Accumulator,
    /// The instruction register. 
    Instruction,
    /// The instruction address register. 
    InstructionAddress,
}

pub fn parse_output_addresses(input: &str) -> Result<usize, String> {
    let res = parse_memory_address(input)
        .map_err(|_| format!("Invalid value passed as output memory address `{input}`. "))?;
    if res > MEMORY_WORDS {
        return Err(format!("Value passed output memory address: {res}, Baby memory address space is {MEMORY_WORDS}. "));
    }
    Ok(res)
}

fn parse_breakpoint_addresses(input: &str) -> Result<usize, String> {
    let res = parse_memory_address(input)
        .map_err(|_| format!("Invalid value passed as breakpoint address `{input}`. "))?;
    if res > MEMORY_WORDS {
        return Err(format!("Value passed as breakpoint address: {res}, Baby memory address space is {MEMORY_WORDS}. "));
    }
    Ok(res)
}

fn parse_memory_address(value: &str) -> Result<usize, ParseIntError> {
    let parse_res = match value {
        v if v.starts_with("0x") => usize::from_str_radix(&v.replace("0x", ""), 16),
        v if v.starts_with("0o") => usize::from_str_radix(&v.replace("0o", ""), 8),
        v if v.starts_with("0b") => usize::from_str_radix(&v.replace("0b", ""), 2),
        v => usize::from_str_radix(&v.replace("0d", ""), 10),
    }?;
    Ok(parse_res)
}
