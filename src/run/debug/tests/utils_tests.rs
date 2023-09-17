use crate::args::Registers;
use super::utils;
use baby_emulator::core::MEMORY_WORDS;


#[test]
fn test_parse_register() {
    assert_eq!(utils::parse_register("   accumulator   "), Ok(Registers::Accumulator));
    assert_eq!(utils::parse_register("   instruction   "), Ok(Registers::Instruction));
    assert_eq!(utils::parse_register("   InstructionAddress   "), Ok(Registers::InstructionAddress));
    assert_eq!(utils::parse_register("   foo   "), Err(format!("foo")));
}

#[test]
fn test_parse_registers() {
    assert_eq!(
        utils::parse_registers("   accumulator   ,  instruction  ,  InstructionAddress   "),
        Ok(vec![Registers::Accumulator, Registers::Instruction, Registers::InstructionAddress])
    );
    assert_eq!(
        utils::parse_registers("   accumulator   ,  instruction  ,  InstructionAddress,  foo   "),
        Err(format!("foo"))
    );
}

#[test]
fn test_parse_memory_address() {
    assert_eq!(utils::parse_memory_address("   0xa   "), Ok(10));
    assert_eq!(utils::parse_memory_address("   0O12   "), Ok(10));
    assert_eq!(utils::parse_memory_address("   0b1010   "), Ok(10));
    assert_eq!(utils::parse_memory_address("   0d10   "), Ok(10));
    assert_eq!(utils::parse_memory_address("   10   "), Ok(10));
    assert_eq!(utils::parse_memory_address("   foo   "), Err(format!("foo")));
    match utils::parse_memory_address("   33   ") {
        Err(e) => assert_eq!(e, format!("Value passed output memory address: 33, Baby memory address space is {MEMORY_WORDS}. ")),
        Ok(_) => panic!("Incorect memory address sucessfully parsed. ")
    }
}

#[test]
fn test_parse_memory_value() {
    assert_eq!(utils::parse_memory_value("   0xa   "), Ok(10));
    assert_eq!(utils::parse_memory_value("   0O12   "), Ok(10));
    assert_eq!(utils::parse_memory_value("   0b1010   "), Ok(10));
    assert_eq!(utils::parse_memory_value("   0d10   "), Ok(10));
    assert_eq!(utils::parse_memory_value("   10   "), Ok(10));
    assert_eq!(utils::parse_memory_value("   foo   "), Err(format!("foo")));
}

#[test]
fn test_parse_instruction() {
    assert_eq!(utils::parse_instruction("   0xa   "), Ok(10));
    assert_eq!(utils::parse_instruction("   0O12   "), Ok(10));
    assert_eq!(utils::parse_instruction("   0b1010   "), Ok(10));
    assert_eq!(utils::parse_instruction("   0d10   "), Ok(10));
    assert_eq!(utils::parse_instruction("   10   "), Ok(10));
    assert_eq!(utils::parse_instruction("   foo   "), Err(format!("foo")));
}

#[test]
fn test_parse_memory_addresses() {
    assert_eq!(
        utils::parse_memory_addresses(format!("   0xa   ,  0O12  ,  0b1010   ,   0d10   ,   10   ")),
        Ok(vec![10, 10, 10, 10, 10])
    );
    assert_eq!(
        utils::parse_memory_addresses(format!("   10   ,   foo   ")),
        Err(format!("Invalid value passed as output memory address `foo`. "))
    );
    match utils::parse_memory_addresses(format!("   10   ,   33   ")) {
        Err(e) => assert!(e.starts_with("Invalid value passed as output memory address")),
        Ok(_) => panic!("Invalid arg to parse memory addresses succeeded. ")
    }
}
