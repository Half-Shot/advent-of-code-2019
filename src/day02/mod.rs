use conv::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn main() -> std::io::Result<()> {
    let mut tape = read_tape_from_memory()?;
    let mut pos: usize = 0;
    let mut opcode = 0;
    let mut acc: u32 = 0;
    loop {
        if opcode > 2 {
            panic!("Wrong opcode");
        }

        if opcode == 0 {
            opcode = tape[pos];
            println!("New op code {}", opcode);
            pos += 1;
        }

        if opcode == 99 {
            break;
        }

        acc = tape[tape[pos] as usize] as u32;
        pos += 1;

        let t = tape[tape[pos] as usize] as u32;

        println!("1.acc {}", acc);

        if opcode == 1 {
            acc += t;
        }

        if opcode == 2 {
            acc *= t;
        }

        println!("1.acc {}", acc);

        pos += 1;
        tape[tape[pos] as usize] = acc;
        pos += 1;
        opcode = 0
    }
    for i in tape.iter() {
        print!("{},", i);
    }
    Ok(())
}

pub fn read_tape_from_memory() -> std::io::Result<[u32; 255]> {
    let mut file = File::open("./input/day02.txt")?;
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