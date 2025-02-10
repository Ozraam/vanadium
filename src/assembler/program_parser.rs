use nom::types::CompleteStr;

use super::instruction_parser::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for instruction in &self.instructions {
            bytes.extend(instruction.to_bytes());
        }
        bytes
    }
}

named!(
    pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
        // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
