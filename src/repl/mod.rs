use nom::types::CompleteStr;

use crate::assembler::program_parser::program;

use super::vm::VM;
use std;
use std::io::{self, Write};
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
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
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();
                    println!("Parsed program: {:?}", bytecode);
                    // TODO: Make a function to let us add bytes to the VM
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
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
