#[derive(Debug)]
pub struct Opcode {
    pub instruction: u8,
    pub d: u8,
    pub w: u8,
    pub m: u8,
    pub reg: u8,
    pub rm: u8,
    pub pc_displacement: u16,
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        let b4_instruction = opcode >> 12 & 0b0000_1111;
        let b6_instruction = opcode >> 10 & 0b0011_1111;

        if b4_instruction == 0b1011 {
            let byte1 = (opcode >> 8) as u8;
            Opcode {
                instruction: b4_instruction as u8,
                d: 0,
                w: byte1 >> 3 & 1,
                m: 0,
                reg: byte1 & 0b0000_0111,
                rm: 0,
                pc_displacement: 1,
            }
        } else {
            Opcode {
                instruction: b6_instruction as u8,
                d: (opcode >> 9 & 1) as u8,
                w: (opcode >> 8 & 1) as u8,
                m: (opcode >> 6 & 0b11) as u8,
                reg: (opcode >> 3 & 0b111) as u8,
                rm: (opcode & 0b111) as u8,
                pc_displacement: 2,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //0x89d9 => mov cx, bx
    #[test]
    fn it_should_return_instruction() {
        let instruction = 0x89d9;
        let opcode = Opcode::new(instruction);

        assert_eq!(opcode.instruction, 0b100010);
    }

    #[test]
    fn it_should_return_d() {
        let instruction = 0x89d9;
        let opcode = Opcode::new(instruction);

        assert_eq!(opcode.d, 0x0);
    }

    #[test]
    fn it_should_return_w() {
        let instruction = 0x89d9;
        let opcode = Opcode::new(instruction);

        assert_eq!(opcode.w, 0x1);
    }

    #[test]
    fn it_should_return_m() {
        let instruction = 0x89d9;
        let opcode = Opcode::new(instruction);

        assert_eq!(opcode.m, 0b11);
    }

    #[test]
    fn it_should_return_reg() {
        let instruction = 0x89d9;
        let opcode = Opcode::new(instruction);

        assert_eq!(opcode.reg, 0b011);
    }

    #[test]
    fn it_should_return_rm() {
        let instruction = 0x89d9;
        let opcode = Opcode::new(instruction);

        assert_eq!(opcode.rm, 0b001);
    }
}
