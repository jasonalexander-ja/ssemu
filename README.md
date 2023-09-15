# SSEM Emulator 

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

This is a CLI tool for running emulations of the Manchester 
Small-Scale Experimental Machine "Baby", the worlds first stored
program computer, providing a familiar albeit primitive 
programming environment. 

# Installation 

Ensure you have cargo installed. 

```
cargo install ssemu
```

# Useage

```
Usage: ssemu <COMMAND>

Commands:
  assemble  Assemble an asm source file to a binary file
  run       Load and run a source file
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Assemble

```
Usage: ssemu assemble [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>    The input asm file
  -o, --output <OUTPUT>  The output binary dump (defaults to input + .bin)
      --og-notation      Use original notation for asm instructions
  -h, --help             Print help
```

## Run

```
Usage: ssemu run [OPTIONS] <SRC>

Arguments:
  <SRC>  The source file to execute from

Options:
      --exe-from <EXE_FROM>        The format of the file to execute from [default: bin] [possible values: asm, bin]
      --og-notation                Use original notation for asm instructions if running from asm
      --output-model               Output whole `model` including registers & memory when execution stops or breakpoint encountered
      --output-addr <OUTPUT_ADDR>  Memory addresses to output when execution stops or breakpoint encountered
      --output-regs <OUTPUT_REGS>  Registers to output when execution stops or breakpoint encountered [possible values: accumulator, instruction, instruction-address]
      --break-addr <BREAK_ADDR>    Addresses where to break & output the state of the core
  -h, --help                       Print help (see more with '--help')
```
