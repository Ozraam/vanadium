use crate::assembler::program_parser::program;

use super::vm::VM;
use std;
use std::io::{self, Write};
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl Default for REPL {
    fn default() -> REPL {
        REPL::new()
    }
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Vanadium! This is a REPL !");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Goodbye! We hope you had fun!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    self.vm.display_program_as_hex();
                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    self.vm.display_registers_square();
                    println!("End of Register Listing")
                }
                ".help" => {
                    println!("Vanadium REPL Help");
                    println!(".help - Show this help message");
                    println!(".quit - Quit the REPL");
                    println!(".history - Show command history");
                    println!(".program - Show the program in the VM");
                    println!(".registers - Show the contents of the registers");
                    println!(".inspect - Show the VM state");
                    println!(".help_instruction - Show the Vanadium instruction set");
                }
                ".inspect" => {
                    println!("Inspecting the VM");
                    println!("{:?}", self.vm);
                }
                ".help_instruction" => {
                    println!("Vanadium Instruction Set");
                    println!("LOAD <register> <value> - Load a value into a register");
                    println!("ADD <register1> <register2> <register3> - Add the values in register2 and register3 and store the result in register1");
                    println!("SUB <register1> <register2> <register3> - Subtract the values in register2 and register3 and store the result in register1");
                    println!("MUL <register1> <register2> <register3> - Multiply the values in register2 and register3 and store the result in register1");
                    println!("DIV <register1> <register2> <register3> - Divide the values in register2 and register3 and store the result in register1");
                    println!("HLT - Halt the program");
                    println!("JMP <value> - Jump to a specific location in the program");
                    println!("JMPF <value> - Jump forward a specific number of instructions");
                    println!("JMPB <value> - Jump backward a specific number of instructions");
                    println!("EQ <register1> <register2> <register3> - Compare the values in register2 and register3 and store 1 in register1 if they are equal, otherwise store 0");
                    println!("NEQ <register1> <register2> <register3> - Compare the values in register2 and register3 and store 1 in register1 if they are not equal, otherwise store 0");
                    println!("GT <register1> <register2> <register3> - Compare the values in register2 and register3 and store 1 in register1 if register2 is greater than register3, otherwise store 0");
                    println!("LT <register1> <register2> <register3> - Compare the values in register2 and register3 and store 1 in register1 if register2 is less than register3, otherwise store 0");
                    println!("GTE <register1> <register2> <register3> - Compare the values in register2 and register3 and store 1 in register1 if register2 is greater than or equal to register3, otherwise store 0");
                    println!("LTE <register1> <register2> <register3> - Compare the values in register2 and register3 and store 1 in register1 if register2 is less than or equal to register3, otherwise store 0");
                    println!("NOP - Do nothing");
                    println!("End of Instruction Set");
                }
                _ => {
                    let parsed_program = match program(buffer.into()) {
                            Ok((_, program)) => program,
                            Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };

                    println!("Parsed program: {:?}", parsed_program);
                    
                    self.vm.program.append(&mut parsed_program.to_bytes());
                    self.vm.run_once();
                }
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
