use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use hack_assembler::{Instruction, Parser};

fn main() {
    let mut args = std::env::args();
    args.next();

    let mut ram_counter: u16 = 16;
    let mut cur_line_number: u16 = 0;

    let file_name = args.next().unwrap_or_else(|| {
        eprintln!("ERROR: no filename supplied.");
        std::process::exit(1);
    });

    let file = fs::read_to_string(&file_name).unwrap_or_else(|err| {
        eprintln!("ERROR: {}: {}", file_name, err);
        std::process::exit(2);
    });

    // first pass
    let parser = Parser::build(&file).unwrap_or_else(|err| {
        eprintln!("ERROR: {}: {}", file_name, err);
        std::process::exit(3);
    });

    let instructions = parser.collect::<Vec<Instruction>>();

    // build symbol table
    let mut symbols_to_labels = HashMap::new();
    for ins in instructions.iter() {
        match ins {
            Instruction::A(sym) => {
                symbols_to_labels.entry(sym).or_insert(false);
            }
            Instruction::L(sym) => {
                symbols_to_labels.insert(sym, true);
            }
            Instruction::C(_) => {}
        }
    }

    // bind symbols to memory addresses and generate corresponding binary instruction
    let mut sym_to_addr = HashMap::new();

    for ins in instructions.iter() {
        match ins {
            Instruction::A(sym) => {
                cur_line_number += 1;
                let is_numeric = sym.parse::<u16>().is_ok();
                let is_label = symbols_to_labels.get(sym).unwrap_or(&false);
                if !is_numeric && !is_label {
                    let address = resolve_symbol_to_address(sym, ram_counter);
                    sym_to_addr.entry(sym).or_insert_with(|| {
                        if address == ram_counter {
                            ram_counter += 1;
                        }
                        address
                    });
                };
            }
            Instruction::L(sym) => {
                sym_to_addr.insert(sym, cur_line_number);
            }
            Instruction::C(_) => {
                cur_line_number += 1;
            }
        }
    }

    println!("Assembling: {file_name}");

    let file_name = file_name.strip_suffix(".asm").expect("Wrong file type!");
    let mut binary = File::create(format!("{}.hack", file_name)).unwrap_or_else(|err| {
        eprintln!("ERROR: {}", err);
        std::process::exit(4);
    });

    for ins in instructions.iter() {
        match ins {
            Instruction::A(sym) => {
                let address = sym.parse().ok().unwrap_or_else(|| {
                    sym_to_addr
                        .get(sym)
                        .expect(&format!(
                            "Expected memory address to be binded to sym: {}",
                            sym
                        ))
                        .clone()
                });
                let bin = format!("{:0>16b}\n", address & 0b0111111111111111);
                binary.write_all(bin.as_bytes()).unwrap_or_else(|err| {
                    eprintln!("ERROR: {}", err);
                    std::process::exit(5);
                });
            }
            Instruction::C(c_instruction) => {
                let c = c_instruction.comp;
                let d = c_instruction.dest.unwrap_or("");
                let j = c_instruction.jump.unwrap_or("");
                let bin = format!("111{}{}{}\n", comp(c), dest(d), jump(j));
                binary.write_all(bin.as_bytes()).unwrap_or_else(|err| {
                    eprintln!("ERROR: {}", err);
                    std::process::exit(5);
                });
            }
            _ => {}
        }
    }
}

fn dest(dest: &str) -> String {
    let m = if dest.contains("M") { "1" } else { "0" };
    let d = if dest.contains("D") { "1" } else { "0" };
    let a = if dest.contains("A") { "1" } else { "0" };
    format!("{a}{d}{m}")
}

fn jump(jump: &str) -> &str {
    match jump {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "000",
    }
}

fn comp(comp: &str) -> &str {
    match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => panic!("Invalid comp"),
    }
}

fn resolve_symbol_to_address(sym: &str, free_address: u16) -> u16 {
    match sym {
        "R0" | "SP" => 0,
        "R1" | "LCL" => 1,
        "R2" | "ARG" => 2,
        "R3" | "THIS" => 3,
        "R4" | "THAT" => 4,
        "R5" => 5,
        "R6" => 6,
        "R7" => 7,
        "R8" => 8,
        "R9" => 9,
        "R10" => 10,
        "R11" => 11,
        "R12" => 12,
        "R13" => 13,
        "R14" => 14,
        "R15" => 15,
        "SCREEN" => 16384,
        "KBD" => 24576,
        _ => free_address,
    }
}
