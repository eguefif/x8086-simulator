use crate::opcode::Opcode;
use crate::registers::Registers;

pub struct Cpu {
    pub memory: [u8; 0xFFF],
    pub registers: Registers,
    pub pc: u16,
}

impl Cpu {
    pub fn new(data: [u8; 0xFFF]) -> Cpu {
        Cpu {
            memory: data,
            registers: Registers::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        println!("bits 16");
        println!();
        loop {
            let opcode: Opcode = self.get_opcode();
            match (opcode.instruction, opcode.w) {
                (0b100010, 0) => self.mov8(opcode),
                (0b100010, 1) => self.mov16(opcode),
                _ => println!("Unknow opcode: {:?}", opcode),
            }
            if self.pc == 0xFFF || self.memory[self.pc as usize] == 0 {
                break;
            }
        }
    }

    fn mov16(&mut self, opcode: Opcode) {
        print!("mov ");
        if opcode.d == 1 {
            self.registers.get_reg16_from_opcode(opcode.reg);
            print!(", ");
            self.registers.get_reg16_from_opcode(opcode.rm);
        } else {
            self.registers.get_reg16_from_opcode(opcode.rm);
            print!(", ");
            self.registers.get_reg16_from_opcode(opcode.reg);
        }
        println!();
    }

    fn mov8(&mut self, opcode: Opcode) {
        print!("mov ");
        if opcode.d == 1 {
            self.registers.get_reg8_from_opcode(opcode.reg);
            print!(", ");
            self.registers.get_reg8_from_opcode(opcode.rm);
        } else {
            self.registers.get_reg8_from_opcode(opcode.rm);
            print!(", ");
            self.registers.get_reg8_from_opcode(opcode.reg);
        }
        println!();
    }

    fn get_opcode(&mut self) -> Opcode {
        let position = self.pc as usize;
        self.pc += 2;
        let opcode = (self.memory[position] as u16) << 8 | self.memory[position + 1] as u16;
        Opcode::new(opcode)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        let data = [0; 0xFFF];
        Self::new(data)
    }
}
