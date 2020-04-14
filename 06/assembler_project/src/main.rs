
mod parser;
use std::env;
use std::fs::File;
use std::io::Read;

fn read_lines_from_file(file_name : &String, line_terminator : &String) -> Vec<String>{
    let mut file = File::open(file_name).unwrap_or_else(|e| {
        panic!("Error opening file {:?}\n", e);
    });

    let mut raw_string = String::new();

    let val = file.read_to_string(&mut raw_string).unwrap_or_else(|e| {
        panic!("Error reading file {:?}\n", e);
    });

    let lines : Vec<String> = raw_string.split(line_terminator).map(|s| s.to_string()).collect();

    return lines;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let assembly_file_name = &args[1];
    let line_terminator = &args[2];
    let mut lines= read_lines_from_file(assembly_file_name, line_terminator);

    let a = parser::Parser::new(&lines);
}
