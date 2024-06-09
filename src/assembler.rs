use std::collections::HashMap;

#[derive(Debug)]
pub struct AInstruction {
    value: Vec<char>,
}

impl AInstruction {
    pub fn new(value: Vec<char>) -> Self {
        Self { value }
    }

    pub fn machine_code(
        self,
        symbol_table: &mut HashMap<String, u16>,
        variable_number: u16,
    ) -> (String, u16) {
        let symbol = self.value.iter().collect::<String>();
        return if let Ok(val) = symbol.parse::<u16>() {
            // If the symbol is value then push it's binary representation
            (format!("{:016b}", val), variable_number)
        } else if symbol_table.contains_key(&symbol) {
            // Found the symbol in the symbol table
            let val = symbol_table.get(&symbol).unwrap();
            (format!("{:016b}", val), variable_number)
        } else {
            // Create a new variable in the symbol table
            symbol_table.insert(symbol, variable_number);
            (format!("{:016b}", variable_number), variable_number + 1)
        };
    }
}

#[derive(Debug)]
pub struct PseudoInstruction {
    pub label: Vec<char>,
}

#[derive(Debug)]
pub struct CInstruction {
    pub dest: Vec<char>,
    pub comp: Vec<char>,
    pub jmp: Vec<char>,
}

impl CInstruction {
    pub fn new() -> Self {
        Self {
            comp: vec![],
            jmp: vec![],
            dest: vec![],
        }
    }

    pub fn machine_code(self) -> String {
        let mut val = 0b111u16 << 13;
        if self.dest.contains(&'M') {
            val |= 0b1 << 3;
        }
        if self.dest.contains(&'D') {
            val |= 0b10 << 3;
        }
        if self.dest.contains(&'A') {
            val |= 0b100 << 3;
        }

        let jmp = self.jmp.iter().collect::<String>();
        val |= match jmp.as_str() {
            "JGT" => 1,
            "JEQ" => 2,
            "JGE" => 3,
            "JLT" => 4,
            "JNE" => 5,
            "JLE" => 6,
            "JMP" => 7,
            _ => 0,
        };

        if self.comp.contains(&'M') {
            val |= 0b1 << 12;
        }

        let comp = self
            .comp
            .iter()
            .filter(|&x| !x.is_whitespace())
            .collect::<String>();
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
            _ => 0,
        } << 6;

        format!("{:016b}", val)
    }
}

#[derive(Debug)]
pub enum Instruction {
    A(AInstruction),
    C(CInstruction),
    Pseudo(PseudoInstruction),
}
