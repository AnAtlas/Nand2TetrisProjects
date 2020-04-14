
mod parser;
mod symbol_table;
mod code_generator;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use symbol_table::SymbolTable;

fn read_lines_from_file(file_name : &String) -> Vec<String>{
    let mut file = File::open(file_name).unwrap_or_else(|_e| {
        panic!("Error opening file {:?}\n", file_name);
    });

    let mut raw_string = String::new();

    file.read_to_string(&mut raw_string).unwrap_or_else(|e| {
        panic!("Error reading file {:?}\n", e);
    });

    raw_string = raw_string.replace("\r\n\r\n", "\r\n");

    return raw_string.lines().map(|s| s.trim().to_string()).collect::<Vec<String>>();
}

fn remove_comments_from_lines(lines : &Vec<String>) -> Vec<String> {
    let mut ret_vec = Vec::new();
    for line in lines {
        if line.starts_with("//") {
            continue;
        }
        if let Some(comment_index) = line.find("//") {
            let string : String = String::from(&line[..comment_index]).trim().to_string();
            ret_vec.push(string);
        }
        else {
            ret_vec.push(line.to_string());
        }
    }
    ret_vec
}

fn write_lines_to_file(file_name : &str, lines : &Vec<u16>) {
    let mut file = File::create(file_name).unwrap_or_else(|_e| {
        panic!("Error creating file {:?}\n", file_name);
    });
    for line in lines {
        for index in (0..16).rev() {
            if line & (1 << index) > 0 {
                file.write("1".as_ref()).unwrap();
            }
            else {
                file.write("0".as_ref()).unwrap();
            }
        }
        file.write_all(b"\r\n").unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let assembly_file_name = &args[1];
    let output_file_name = &args[2];
    let lines = remove_comments_from_lines(&read_lines_from_file(assembly_file_name));

    let mut a = parser::Parser::new(&lines).unwrap_or_else(|e| {
        panic!("Error creating parser {}\n", e);
    });

    let mut symbol_table = SymbolTable::new(0);
    symbol_table.pass_1(&mut a);
    let lines = symbol_table.pass_2(&lines);
    let mut a = parser::Parser::new(&lines).ok().unwrap();
    let machine_lines = code_generator::generate_machine_lines(&mut a);

    write_lines_to_file(output_file_name, &machine_lines);
    println!("Successfully wrote file {}", output_file_name);
}
