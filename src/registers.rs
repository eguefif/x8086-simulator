pub struct Registers {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,

    pub ah: u8,
    pub bh: u8,
    pub ch: u8,
    pub dh: u8,

    pub al: u8,
    pub bl: u8,
    pub cl: u8,
    pub dl: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            ah: 0,
            bh: 0,
            ch: 0,
            dh: 0,
            al: 0,
            bl: 0,
            cl: 0,
            dl: 0,
        }
    }

    pub fn get_mem(&mut self, rm: u8, value: u16) {
        print!("[");
        match rm {
            0b000 => print!("bx + si"),
            0b001 => print!("bx + di"),
            0b010 => print!("bp + si"),
            0b011 => print!("bp + di"),
            0b100 => print!("si"),
            0b101 => print!("di"),
            0b110 => print!("bp"),
            0b111 => print!("bx"),
            _ => print!("Unknow mod {:b}", rm),
        }
        if value != 0 {
            print!("+ {}", value);
        }
        print!("]");
    }

    pub fn get_reg8_from_opcode(&mut self, opcode: u8) {
        match opcode {
            0b0 => print!("al"),
            0b1 => print!("cl"),
            0b10 => print!("dl"),
            0b11 => print!("bl"),
            0b100 => print!("ah"),
            0b101 => print!("ch"),
            0b110 => print!("dh"),
            0b111 => print!("bh"),
            _ => print!("Unknown reg"),
        }
    }

    pub fn get_reg16_from_opcode(&mut self, opcode: u8) {
        match opcode {
            0b0 => print!("ax"),
            0b1 => print!("cx"),
            0b10 => print!("dx"),
            0b11 => print!("bx"),
            0b100 => print!("sp"),
            0b101 => print!("bp"),
            0b110 => print!("si"),
            0b111 => print!("di"),
            _ => print!("Unknown reg"),
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
