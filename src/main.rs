use crate::cpu::Cpu;
use std::fs::File;
use std::io::prelude::*;

pub mod cpu;
pub mod opcode;
pub mod registers;

fn main() {
    let mut file = File::open("./bytes").unwrap();
    let mut content = [0; 0xFFF];
    let _ = file.read(&mut content).unwrap();
    let mut cpu = Cpu::new(content);
    cpu.run();
}
