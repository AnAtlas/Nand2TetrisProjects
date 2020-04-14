
use std::collections::HashMap;
use crate::parser::*;

type Symbol = String;
type Address = u16;

pub struct SymbolTable {
    pub symbol_map : HashMap<Symbol, Address>,
    ram_address : Address,
    rom_address : Address,
}

fn preload_internal_symbols(table : &mut SymbolTable) {
    table.add_entry("SP", table.ram_address);
    table.add_entry("LCL", table.ram_address + 1);
    table.add_entry("ARG", table.ram_address + 2);
    table.add_entry("THIS", table.ram_address + 3);
    table.add_entry("THAT", table.ram_address + 4);
    for x in 0 .. 16 {
        let mut s = String::from("R");
        s.push_str(x.to_string().as_ref());
        table.add_entry(s.as_ref(), table.ram_address + x);
    }
    table.ram_address += 16;
    table.add_entry("SCREEN", 0x4000);
    table.add_entry("KBD", 0x6000);
}

impl SymbolTable {
    pub fn new(ram_address : Address) -> SymbolTable {
        let mut table = SymbolTable {
            symbol_map : HashMap::new(),
            ram_address,
            rom_address : 0,
        };
        preload_internal_symbols(&mut table);
        table
    }

    fn add_ram_entry(&mut self, symbol : &str) {
        self.add_entry(symbol, self.ram_address);
        self.ram_address += 1;
    }

    fn add_rom_entry(&mut self, symbol : &str) {
        self.add_entry(symbol, self.rom_address);
    }

    fn add_entry(&mut self, symbol : &str, address : Address) {
        if !self.symbol_map.contains_key(symbol) {
            self.symbol_map.insert(symbol.to_string(), address);
        }
    }

    fn get_address(&self, symbol : &str) -> Option<&Address> {
        self.symbol_map.get(symbol)
    }

    //Find and record all ROM addresses
    pub fn pass_1(&mut self, parser : &mut Parser) {
        parser.reset();
        loop {
            if let Some(c_type) = parser.command_type() {
                match c_type {
                    CommandType::A => self.rom_address += 1,
                    CommandType::C => self.rom_address += 1,
                    CommandType::L => {
                        self.add_rom_entry(parser.symbol().unwrap().as_str());
                    }
                }
            }
            else{
                println!("ERROR");
            }
            if parser.advance().is_err() {
                break;
            }
        }
        parser.reset();
        loop {
            if let Some(c_type) = parser.command_type() {
                match c_type {
                    CommandType::A => {
                        if let Some(symbol) = parser.symbol() {
                            if symbol.parse::<u16>().is_err() {
                                if !self.symbol_map.contains_key(&symbol) {
                                    self.add_ram_entry(&symbol);
                                }
                            }
                        }
                    },
                    CommandType::C => (),
                    CommandType::L => (),
                }
            }
            else{
                println!("ERROR");
            }
            if parser.advance().is_err() {
                break;
            }
        }
    }

    pub fn pass_2(&mut self, lines : &Vec<String>) -> Vec<String> {
        let mut new_lines = Vec::new();
        for line in lines {
            let mut new_line = Some(String::from(line));
            for symbol in self.symbol_map.keys() {
                if line.contains(symbol.as_str()) {
                    if line.contains('@') && line.len() == symbol.len() + 1{
                        new_line = Some(line.replace(symbol.as_str(),
                                                self.get_address(symbol).unwrap().to_string().as_ref()));
                        break;
                    }
                    else {
                        new_line = None;
                    }
                }
            }
            if let Some(line) = new_line {
                new_lines.push(String::from(line));
            }
        }
        new_lines
    }
}