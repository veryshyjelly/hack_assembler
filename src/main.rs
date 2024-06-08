use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::parser::{CInstruction, Instruction, Parser};

mod lexer;
mod parser;

fn main() {
    let file_name = args().nth(1).unwrap();
    let mut data = String::new();
    let _ = File::open(&file_name).unwrap().read_to_string(&mut data).unwrap();
    let content = data.to_uppercase().chars().collect::<Vec<char>>();

    let mut symbol_table = HashMap::from(
        [("R0".into(), 0), ("R1".into(), 1), ("R2".into(), 2), ("R3".into(), 3),
            ("R4".into(), 4), ("R5".into(), 5), ("R6".into(), 6), ("R7".into(), 7),
            ("R8".into(), 8), ("R9".into(), 9), ("R10".into(), 10), ("R11".into(), 11),
            ("R12".into(), 12), ("R13".into(), 13), ("R14".into(), 14), ("R15".into(), 15),
            ("SCREEN".into(), 16384), ("KBD".into(), 24576), ("SP".into(), 0),
            ("LCL".into(), 1), ("ARG".into(), 2), ("THIS".into(), 3), ("THAT".into(), 4)]
    );

    let mut parser = Parser::new(&content);
    let mut instructions = vec![];
    while let Some(instruction) = parser.next_instruction() {
        instructions.push(instruction);
    }

    let mut line_number = 0;
    for instr in &instructions {
        if let Instruction::Pseudo(inst) = instr {
            let symbol = inst.label.iter().collect::<String>();
            symbol_table.insert(symbol, line_number);
        } else {
            line_number += 1;
        }
    }

    let mut variable_number = 16u16;
    let mut assembled = vec![];

    for instr in &instructions {
        match instr {
            Instruction::A(inst) => {
                let symbol = inst.value.iter().collect::<String>();
                if let Ok(val) = symbol.parse::<u16>() {
                    // If the symbol is value then push it's binary representation
                    assembled.push(format!("{:016b}", val));
                } else {
                    // Else look for the value in symbol table
                    if !symbol_table.contains_key(&symbol) {
                        symbol_table.insert(symbol, variable_number);
                        assembled.push(format!("{:016b}", variable_number));
                        variable_number += 1;
                    } else {
                        // Create a new variable in the symbol table
                        let val = symbol_table.get(&symbol).unwrap();
                        assembled.push(format!("{:016b}", val));
                    }
                }
            }
            Instruction::C(inst) => {
                let val = assemble_c(inst);
                assembled.push(format!("{:016b}", val));
            }
            _ => {}
        }
    }

    let assembled_string = assembled.join("\n");
    println!("{}", assembled_string);

    let new_file = Path::new(&file_name);
    File::create(new_file.with_extension("hack"))
        .unwrap()
        .write_fmt(format_args!("{}", assembled_string))
        .unwrap();
}

fn assemble_c(inst: &CInstruction) -> u16 {
    let mut val = 0b111u16 << 13;
    if inst.dest.contains(&'M') {
        val |= 0b1 << 3;
    }
    if inst.dest.contains(&'D') {
        val |= 0b10 << 3;
    }
    if inst.dest.contains(&'A') {
        val |= 0b100 << 3;
    }

    let jmp = inst.jmp.iter().collect::<String>();
    val |= match jmp.as_str() {
        "JGT" => 1,
        "JEQ" => 2,
        "JGE" => 3,
        "JLT" => 4,
        "JNE" => 5,
        "JLE" => 6,
        "JMP" => 7,
        _ => 0
    };

    if inst.comp.contains(&'M') {
        val |= 0b1 << 12;
    }

    let comp = inst.comp.iter()
        .filter(|&x| !x.is_whitespace()).collect::<String>();
    val |= match comp.as_str() {
        "0" => 0b101010,
        "1" => 0b111111,
        "-1" => 0b111010,
        "D" => 0b001100,
        "A" | "M" => 0b110000,
        "!D" => 0b001101,
        "!A" | "!M" => 0b110001,
        "-D" => 0b001111,
        "-A" | "-M" => 0b110011,
        "D+1" => 0b011111,
        "A+1" | "M+1" => 0b110111,
        "D-1" => 0b001110,
        "A-1" | "M-1" => 0b110010,
        "D+A" | "D+M" | "M+D" | "A+D" => 0b000010,
        "D-A" | "D-M" => 0b010011,
        "A-D" | "M-D" => 0b000111,
        "D&A" | "D&M" | "M&D" | "A&D" => 0,
        "D|M" | "M|D" | "D|A" | "A|D" => 0b010101,
        _ => 0
    } << 6;

    val
}
