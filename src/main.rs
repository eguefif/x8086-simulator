use crate::cpu::Cpu;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub mod cpu;
pub mod opcode;
pub mod registers;

fn main() {
    let filename = get_filename();
    let mut file = File::open(filename).unwrap();
    let mut content = [0; 0xFFF];
    let _ = file.read(&mut content).unwrap();
    let mut cpu = Cpu::new(content);
    cpu.run();
}

fn get_filename() -> String {
    let arg: Vec<String> = env::args().collect();
    String::from(&arg[1])
}
