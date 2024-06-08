use crate::lexer::Lexer;

#[derive(Debug)]
pub struct AInstruction {
    pub value: Vec<char>,
}

#[derive(Debug)]
pub struct CInstruction {
    pub dest: Vec<char>,
    pub comp: Vec<char>,
    pub jmp: Vec<char>,
}

#[derive(Debug)]
pub struct PseudoInstruction {
    pub label: Vec<char>,
}

#[derive(Debug)]
pub enum Instruction {
    A(AInstruction),
    C(CInstruction),
    Pseudo(PseudoInstruction),
}

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
            let instruction = AInstruction { value: s.to_vec() };

            Some(Instruction::A(instruction))
        } else if f[0] == '(' { // case of a label
            let s = self.lexer.chop_while(|&x| x != ')');
            self.lexer.content = &self.lexer.content[1..];
            let instruction = PseudoInstruction { label: s.to_vec() };

            Some(Instruction::Pseudo(instruction))
        } else { // otherwise c instruction
            let s = self.lexer.next_token().unwrap_or(&[]);
            if s.len() == 0 { return None; }
            let mut instruction = CInstruction {
                dest: vec![],
                comp: vec![],
                jmp: vec![],
            };

            if s[0] == '=' {
                instruction.dest = f.to_vec();
                let x = self.lexer.chop_while(|&x| x != '\n' && x != '\r' && x != ';');
                instruction.comp.extend_from_slice(x);
                self.lexer.trim_left();
                if self.lexer.content.len() > 0 && self.lexer.content[0] == ';' {
                    let t = self.lexer.next_token().unwrap();
                    instruction.jmp = t.to_vec();
                }
            } else if s[0] == ';' {
                let t = self.lexer.next_token().unwrap();
                instruction.comp = f.to_vec();
                instruction.jmp = t.to_vec();
            } else {
                instruction.comp = f.to_vec();
            }

            Some(Instruction::C(instruction))
        }
    }
}