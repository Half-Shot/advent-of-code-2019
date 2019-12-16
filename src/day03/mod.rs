use conv::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn main() -> std::io::Result<()> {

}

pub fn read_tape_from_memory() -> std::io::Result<[u32; 255]> {
    let mut file = File::open("./input/day03.txt")?;
    let mut memory: [u32; 255] = [0; 255];
    let mut buffer = [0; 1];
    let mut accumulator: u32 = 0;
    let mut mem_pos: usize = 0;
    loop {
        let res = file.read(&mut buffer)?;
        if res == 0 {
            println!("End of tape");
            break;
        }

        if buffer[0] == 0x2C {
            println!("{} {}", mem_pos, accumulator);
            memory[mem_pos] = accumulator;
            mem_pos += 1;
            accumulator = 0;
            continue;
        }

        if buffer[0] == 0x0A {
            println!("End of tape");
            break;
        }

        let tape_int: u32 = (buffer[0] as u32) - 48;

        if accumulator != 0 {
            accumulator = (accumulator * 10) + tape_int
        } else {
            accumulator = tape_int
        }
    }
    Ok(memory)
}