
use crate::parser::*;

type MachineCommand = u16;

pub fn dest(dest : String) -> Option<u16> {
    let mut ret : u16 = 0;
    if dest.contains('M') {
        ret += 1;
    }
    if dest.contains('D') {
        ret += 2;
    }
    if dest.contains('A') {
        ret += 4;
    }
    if ret > 0 {
        return Some(ret << 3);
    }
    None
}

pub fn comp(comp : String) -> Option<u16> {
    let ret = match comp.as_str() {
        "0" => Some(0xA80),
        "1" => Some(0xFC0),
        "-1" => Some(0xE80),
        "D" => Some(0x300),
        "A" | "M" => Some(0xC00),
        "!D" => Some(0x340),
        "!A" | "!M" => Some(0xC40),
        "-D" => Some(0x3C0),
        "-A" | "-M" => Some(0xCC0),
        "D+1" => Some(0x7C0),
        "A+1" | "M+1" => Some(0xDC0),
        "D-1" => Some(0x380),
        "A-1" | "M-1" => Some(0xC80),
        "D+A" | "D+M" | "A+D" | "M+D"=> Some(0x80),
        "D-A" | "D-M" => Some(0x4C0),
        "A-D" | "M-D" => Some(0x1C0),
        "D&A" | "D&M" | "A&D" | "M&D" => Some(0x0),
        "D|A" | "D|M" | "A|D" | "M|D" => Some(0x540),
        _ => None,
    };
    if comp.contains("M") && ret.is_some(){
        return Some(ret.unwrap() + 0x1000);
    }
    ret
}

pub fn jump(jump : String) -> Option<u16> {
    return match jump.as_str() {
        "JGT" => Some(1),
        "JEQ" => Some(2),
        "JGE" => Some(3),
        "JLT" => Some(4),
        "JNE" => Some(5),
        "JLE" => Some(6),
        "JMP" => Some(7),
        _ => None,
    }
}

pub fn generate_machine_lines(parser : &mut Parser) -> Vec<MachineCommand> {
    let mut machine_lines = Vec::new();
    loop {
        let mut command : MachineCommand = 0;
        match parser.command_type() {
            Some(CommandType::A) => {
                command = parser.symbol().unwrap().parse::<u16>().unwrap();
            },
            Some(CommandType::C) => {
                if let Some(d) = parser.dest() {
                    if let Some(val) = dest(d) {
                        command += val;
                    }
                }
                if let Some(c) = parser.comp() {
                    if let Some(val) = comp(c) {
                        command += val;
                    }
                }
                if let Some(j) = parser.jump() {
                    if let Some(val) = jump(j) {
                        command += val;
                    }
                }
                command += 0xE000;
            },
            _ => {
                println!("BAD COMMAND");
            }
        }

        machine_lines.push(command);
        if parser.advance().is_err() {
            break;
        }
    }
    machine_lines
}