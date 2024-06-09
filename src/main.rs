use crate::parser::Parser;
use assembler::Instruction;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod assembler;
mod lexer;
mod parser;

use Instruction::{Pseudo, A, C};

fn main() {
    let file_name = args().nth(1).expect("Usage: cargo run <filename>");
    let mut data = String::new();
    let _ = File::open(&file_name)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let content = data.chars().collect::<Vec<char>>();

    let mut symbol_table = HashMap::from([
        ("R0".into(), 0),
        ("R1".into(), 1),
        ("R2".into(), 2),
        ("R3".into(), 3),
        ("R4".into(), 4),
        ("R5".into(), 5),
        ("R6".into(), 6),
        ("R7".into(), 7),
        ("R8".into(), 8),
        ("R9".into(), 9),
        ("R10".into(), 10),
        ("R11".into(), 11),
        ("R12".into(), 12),
        ("R13".into(), 13),
        ("R14".into(), 14),
        ("R15".into(), 15),
        ("SCREEN".into(), 16384),
        ("KBD".into(), 24576),
        ("SP".into(), 0),
        ("LCL".into(), 1),
        ("ARG".into(), 2),
        ("THIS".into(), 3),
        ("THAT".into(), 4),
    ]);

    let mut parser = Parser::new(&content);
    // Convert all the files into instructions using parser
    let mut instructions = vec![];
    while let Some(instruction) = parser.next_instruction() {
        instructions.push(instruction);
    }

    // In first pass get all the labels in the symbol table
    let mut line_number = 0;
    for instr in &instructions {
        if let Pseudo(inst) = instr {
            let symbol = inst.label.iter().collect::<String>();
            symbol_table.insert(symbol, line_number);
        } else {
            line_number += 1;
        }
    }

    let mut variable_number = 16u16;
    let mut machine_codes = vec![];
    let mut code: String;

    for instr in instructions {
        match instr {
            A(inst) => {
                (code, variable_number) = inst.machine_code(&mut symbol_table, variable_number);
                machine_codes.push(code);
            }
            C(inst) => machine_codes.push(inst.machine_code()),
            _ => {}
        }
    }

    let assembled_string = machine_codes.join("\n");
    println!("{}", assembled_string);

    File::create(Path::new(&file_name).with_extension("hack"))
        .expect("Unable to create file")
        .write_fmt(format_args!("{}", assembled_string))
        .expect("Unable to write the file");
}
