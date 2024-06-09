use super::assembler::{AInstruction, CInstruction, Instruction, PseudoInstruction};
use super::lexer::Lexer;

use Instruction::{Pseudo, A, C};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            lexer: Lexer::new(content),
        }
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        let f = self.lexer.next_token().unwrap_or(&[]);
        if f.len() == 0 {
            return None;
        }

        if f[0] == '@' {
            // case of A instruction
            let s = self.lexer.next_token().unwrap_or(&[]);
            let instruction = AInstruction::new(s.to_vec());

            Some(A(instruction))
        } else if f[0] == '(' {
            // case of a label
            let s = self.lexer.next_token().unwrap_or(&[]);
            let closing_braces = self.lexer.next_token().unwrap_or(&[]);
            if closing_braces.len() != 1 || closing_braces[0] != ')' {
                panic!(
                    "Syntax error: expected ')' after {}",
                    s.iter().collect::<String>()
                );
            }
            let instruction = PseudoInstruction { label: s.to_vec() };

            Some(Pseudo(instruction))
        } else {
            // otherwise c instruction
            let s = self.lexer.next_token().unwrap_or(&[]);
            if s.len() == 0 {
                panic!(
                    "Syntax error: expected token after {}",
                    f.iter().collect::<String>()
                );
            }
            let mut instruction = CInstruction::new();

            // There is a destination
            if s[0] == '=' {
                instruction.dest = f.to_vec();
                while !self.lexer.is_empty() && !self.lexer.content[0].is_control() {
                    let token = self.lexer.next_token().unwrap_or(&[]);
                    if token.is_empty() {
                        break;
                    }

                    // There is a jump
                    if token[0] == ';' {
                        let jmp = self.lexer.next_token().unwrap_or(&[]);
                        instruction.jmp = jmp.to_vec();
                        break;
                    } else {
                        instruction.comp.extend_from_slice(token);
                    }
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
