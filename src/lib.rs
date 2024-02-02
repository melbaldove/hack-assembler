use std::{error::Error, str::Lines};

#[derive(Clone)]
pub struct Parser<'a> {
    file: Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn build(file_contents: &str) -> Result<Parser, Box<dyn Error>> {
        let file = file_contents.lines();
        Ok(Parser { file })
    }
}

#[derive(Debug)]
pub struct CInstruction<'a> {
    pub dest: Option<&'a str>,
    pub comp: &'a str,
    pub jump: Option<&'a str>,
}

#[derive(Debug)]
pub enum Instruction<'a> {
    A(&'a str),
    L(&'a str),
    C(CInstruction<'a>),
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = loop {
            match self.file.next() {
                None => return None,
                Some(line) => {
                    let line = line.trim();
                    let first_char = line.chars().nth(0).unwrap_or(' ');
                    if !line.is_empty() && first_char != '/' {
                        break Some(line);
                    }
                }
            }
        };
        let line = line.expect("Should have value");
        parse_instruction(line).ok()
    }
}

fn parse_instruction<'a>(line: &'a str) -> Result<Instruction<'a>, &'static str> {
    if line.trim().is_empty() || line.starts_with('/') {
        return Err("Invalid line");
    }
    let line = line.trim();
    let first_char = line.chars().next().ok_or("should be valid line")?;

    let ins = match first_char {
        '(' => {
            let symbol = line.trim_start_matches('(').trim_end_matches(')');
            Instruction::L(symbol)
        }
        '@' => {
            let symbol = line.trim_start_matches('@');
            Instruction::A(symbol)
        }
        _ => {
            let (mut eq_index, mut semicolon_index) = (None, None);
            for (i, c) in line.char_indices() {
                match c {
                    '=' => eq_index = Some(i),
                    ';' => semicolon_index = Some(i),
                    _ => {}
                }
            }

            let comp_start = eq_index.map(|x| x + 1).unwrap_or(0);
            let comp_end = semicolon_index.unwrap_or(line.len());
            let dest = eq_index.map(|i| &line[0..i]);
            let jump = semicolon_index.map(|i| &line[i + 1..]);
            let comp = &line[comp_start..comp_end];
            Instruction::C(CInstruction { dest, comp, jump })
        }
    };
    Ok(ins)
}

#[cfg(test)]
mod tests {

    use crate::{parse_instruction, Instruction, Parser};

    #[test]
    fn parse_a_instruction() {
        match parse_instruction("@1234").unwrap() {
            Instruction::A(sym) => assert_eq!(sym, "1234"),
            _ => panic!("error: parse_a_instruction() expected an A instruction"),
        }

        match parse_instruction("@symbol").unwrap() {
            Instruction::A(sym) => assert_eq!(sym, "symbol"),
            _ => panic!("error: parse_a_instruction() expected an A instruction"),
        }
    }

    #[test]
    fn parse_l_instruction() {
        match parse_instruction("(LOOP)").unwrap() {
            Instruction::L(sym) => assert_eq!(sym, "LOOP"),
            _ => panic!("error: parse_l_instruction() expected an L instruction"),
        }
    }

    #[test]
    fn parse_c_instruction() {
        match parse_instruction("D=D+1;JLE").unwrap() {
            Instruction::C(c_instruction) => {
                assert_eq!(c_instruction.dest, Some("D"));
                assert_eq!(c_instruction.comp, "D+1");
                assert_eq!(c_instruction.jump, Some("JLE"));
            }
            _ => panic!("error: parse_c_instruction() expected an C instruction"),
        }

        match parse_instruction("M=M-1").unwrap() {
            Instruction::C(c_instruction) => {
                assert_eq!(c_instruction.dest, Some("M"));
                assert_eq!(c_instruction.comp, "M-1");
                assert_eq!(c_instruction.jump, None);
            }
            _ => panic!("error: parse_c_instruction() expected an C instruction"),
        }
        match parse_instruction("A+1").unwrap() {
            Instruction::C(c_instruction) => {
                assert_eq!(c_instruction.dest, None);
                assert_eq!(c_instruction.comp, "A+1");
                assert_eq!(c_instruction.jump, None);
            }
            _ => panic!("error: parse_c_instruction() expected an C instruction"),
        }

        match parse_instruction("0;JMP").unwrap() {
            Instruction::C(c_instruction) => {
                assert_eq!(c_instruction.dest, None);
                assert_eq!(c_instruction.comp, "0");
                assert_eq!(c_instruction.jump, Some("JMP"));
            }
            _ => panic!("error: parse_c_instruction() expected an C instruction"),
        }
    }

    #[test]
    fn parser() {
        let file = "\
@sum
D=M

// some comment
(LOOP)
A=D+1;JMP
M=D

// another comment
@LOOP
0;JMP";

        let parser = Parser::build(file).unwrap();
        let ins = parser.collect::<Vec<Instruction>>();
        assert_eq!(ins.len(), 7);
    }
}
