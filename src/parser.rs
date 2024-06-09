use super::assembler::{AInstruction, CInstruction, Instruction, PseudoInstruction};
use super::lexer::Lexer;

use Instruction::{A, C, Pseudo};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { lexer: Lexer::new(content) }
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        let f = self.lexer.next_token().unwrap_or(&[]);
        if f.len() == 0 { return None; }

        if f[0] == '@' { // case of A instruction
            let s = self.lexer.chop_while(|&x| x != '/' && !x.is_whitespace());
            let instruction = AInstruction::new(s.to_vec());

            Some(A(instruction))
        } else if f[0] == '(' { // case of a label
            let s = self.lexer.chop_while(|&x| x != ')');
            self.lexer.content = &self.lexer.content[1..];
            let instruction = PseudoInstruction { label: s.to_vec() };

            Some(Pseudo(instruction))
        } else { // otherwise c instruction
            let s = self.lexer.next_token().unwrap_or(&[]);
            if s.len() == 0 { return None; }
            let mut instruction = CInstruction::new();

            if s[0] == '=' {
                instruction.dest = f.to_vec();
                // Read full line till there is ;
                let x = self.lexer.chop_while(|&x| !x.is_control() && x != ';');

                // Check for comments across the line
                let mut i = 1;
                while i < x.len() {
                    if x[i] == '/' && x[i - 1] == '/' {
                        i -= 1;
                        break;
                    }
                    i += 1;
                }

                instruction.comp.extend_from_slice(&x[..i]);
                self.lexer.trim_left();

                if !self.lexer.is_empty() && self.lexer.content[0] == ';' {
                    self.lexer.content = &self.lexer.content[1..];
                    let jmp = self.lexer.next_token().unwrap_or(&[]);
                    instruction.jmp = jmp.to_vec();
                }
            } else if s[0] == ';' {
                let jmp = self.lexer.next_token().unwrap_or(&[]);
                instruction.comp = f.to_vec();
                instruction.jmp = jmp.to_vec();
            } else {
                instruction.comp = f.to_vec();
            }

            Some(C(instruction))
        }
    }
}