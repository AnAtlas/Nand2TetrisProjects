extern crate queues;

use queues::Queue;
use self::queues::IsQueue;

#[derive(Copy, Clone, PartialEq)]
pub enum CommandType {
    A,
    C,
    L,
}

pub enum NewParserError {
    ErrorOpeningFile,
    ErrorReadingFile,
    FileEmpty,
    AdvanceError(AdvanceError),
}

#[derive(Debug)]
pub enum ParseError {
    CommandType,
    SymbolStartsWithDigit,
}

pub enum AdvanceError {
    NoMoreCommands,
    ParseError(ParseError),
}

pub struct Parser {
    lines : Queue<String>,
}

impl Parser {
    pub fn new(lines : &Vec<String>) -> Result<Parser, NewParserError> {
        if lines.len() > 0 {
            let mut p = Parser {
                lines: Queue::new(),
            };
            for line in lines {
                if p.lines.add(String::from(line)).is_err() {
                    return Err(NewParserError::ErrorReadingFile);
                }
            }
            return Ok(p);
        }
        Err(NewParserError::FileEmpty)
    }

    pub fn has_more_commands(&self) -> bool {
        self.lines.size() > 1
    }

    fn get_command_type_from_line(line : &str) -> Result<CommandType, ParseError> {
        if line.starts_with('@') {
            if line.chars().next().unwrap().is_digit(10){
                return Err(ParseError::SymbolStartsWithDigit);
            }
            return Ok(CommandType::A);
        }
        if line.contains('=') || line.contains(';') {
            return Ok(CommandType::C);
        }
        return Ok(CommandType::L);
    }

    fn get_symbol_from_line(command_type : &CommandType, line : &str) -> Result<String, ()> {
        match command_type {
            CommandType::A => Ok(String::from(&line[1..])),
            CommandType::L => Ok(String::from(&line[1..line.len() - 1])),
            CommandType::C => Err(()),
        }
    }

    fn get_dest_from_line(command_type : &CommandType, line : &str) -> Result<String, ()> {
        return match command_type {
            CommandType::A => Err(()),
            CommandType::L => Err(()),
            CommandType::C => {
                if let Some(equal_index) = line.find('=') {
                    Ok(String::from(&line[0..equal_index]))
                }
                else {
                    Err(())
                }
            },
        }
    }

    fn get_comp_from_line(command_type : &CommandType, line : &str) -> Result<String, ()> {
        return match command_type {
            CommandType::A => Err(()),
            CommandType::L => Err(()),
            CommandType::C => {
                if let Some(semicolon_index) = line.find(';') {
                    return Ok(String::from(&line[..semicolon_index]));
                }
                if let Some(equal_index) = line.find('=') {
                    return Ok(String::from(&line[equal_index + 1..]));
                }
                Err(())
            },
        }
    }

    fn get_jump_from_line(command_type : &CommandType, line : &str) -> Result<String, ()> {
        return match command_type {
            CommandType::A => Err(()),
            CommandType::L => Err(()),
            CommandType::C => {
                if let Some(semicolon_index) = line.find(';') {
                    return Ok(String::from(&line[semicolon_index + 1 ..]));
                }
                Err(())
            },
        }
    }

    pub fn advance(&mut self) -> Result<(), AdvanceError> {
        if !self.has_more_commands() {
            return Err(AdvanceError::NoMoreCommands);
        }
        if self.lines.remove().is_err() {
            return Err(AdvanceError::NoMoreCommands);
        }
        Ok(())
    }

    pub fn command_type(&self) -> Option<CommandType> {
        if let Ok(line) = self.lines.peek() {
            if let Ok(command_type) = Parser::get_command_type_from_line(line.as_ref()) {
                return Some(command_type);
            }
        }
        None
    }

    pub fn symbol(&self) -> Option<String> {
        if let Ok(line) = self.lines.peek() {
            if let Ok(symbol) = Parser::get_symbol_from_line(&self.command_type().unwrap(), line.as_ref()) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn dest(&self) -> Option<String> {
        if let Ok(line) = self.lines.peek() {
            if let Ok(dest) = Parser::get_dest_from_line(&self.command_type().unwrap(), line.as_ref()) {
                return Some(dest);
            }
        }
        None
    }

    pub fn comp(&self) -> Option<String> {
        if let Ok(line) = self.lines.peek() {
            if let Ok(comp) = Parser::get_comp_from_line(&self.command_type().unwrap(), line.as_ref()) {
                return Some(comp);
            }
        }
        None
    }

    pub fn jump(&self) -> Option<String> {
        if let Ok(line) = self.lines.peek() {
            if let Ok(jump) = Parser::get_jump_from_line(&self.command_type().unwrap(), line.as_ref()) {
                return Some(jump);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, CommandType};

    #[test]
    fn parser_constructor_test() {
        let lines = Vec::new();
        assert!(Parser::new(&lines).is_err());
        let lines = vec!["@i".to_string(), "M=1".to_string(), "@sum".to_string(), "M=0".to_string(), "(LOOP)".to_string()];
        let p = Parser::new(&lines);
        assert!(p.is_ok());
    }

    #[test]
    fn has_more_commands_test() {
        let lines = vec!["@i".to_string()];
        if let Ok(p) = Parser::new(&lines){
            assert!(!p.has_more_commands());
        }
        else {
            assert!(false);
        }

        let lines = vec!["@i".to_string(), "@i".to_string()];
        if let Ok(p) = Parser::new(&lines){
            assert!(p.has_more_commands());
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn command_type_test() {
        let lines = vec!["@i".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.command_type().is_some());
        assert!(p.command_type().unwrap() == CommandType::A);

        let lines = vec!["M=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.command_type().is_some());
        assert!(p.command_type().unwrap() == CommandType::C);

        let lines = vec!["0;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.command_type().is_some());
        assert!(p.command_type().unwrap() == CommandType::C);

        let lines = vec!["(LOOP)".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.command_type().is_some());
        assert!(p.command_type().unwrap() == CommandType::L);
    }

    #[test]
    fn symbol_test() {
        let lines = vec!["@i".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.symbol().is_some());
        assert_eq!(p.symbol().unwrap(), "i".parse::<String>().unwrap());

        let lines = vec!["M=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.symbol().is_none());

        let lines = vec!["0;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.symbol().is_none());

        let lines = vec!["(LOOP)".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.symbol().is_some());
        assert_eq!(p.symbol().unwrap(), "LOOP".parse::<String>().unwrap());
    }

    #[test]
    fn dest_test() {
        let lines = vec!["@i".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_none());

        let lines = vec!["M=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_some());
        assert_eq!(p.dest().unwrap(), "M".parse::<String>().unwrap());

        let lines = vec!["MD=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_some());
        assert_eq!(p.dest().unwrap(), "MD".parse::<String>().unwrap());

        let lines = vec!["AMD=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_some());
        assert_eq!(p.dest().unwrap(), "AMD".parse::<String>().unwrap());

        let lines = vec!["0;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_none());

        let lines = vec!["(LOOP)".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_none());
    }

    #[test]
    fn comp_test() {
        let lines = vec!["@i".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.comp().is_none());

        let lines = vec!["M=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.comp().is_some());
        assert_eq!(p.comp().unwrap(), "1".parse::<String>().unwrap());

        let lines = vec!["M=D+1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.comp().is_some());
        assert_eq!(p.comp().unwrap(), "D+1".parse::<String>().unwrap());

        let lines = vec!["0;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.comp().is_some());
        assert_eq!(p.comp().unwrap(), "0".parse::<String>().unwrap());

        let lines = vec!["D+1;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.comp().is_some());
        assert_eq!(p.comp().unwrap(), "D+1".parse::<String>().unwrap());

        let lines = vec!["(LOOP)".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.dest().is_none());
    }

    #[test]
    fn jump_test() {
        let lines = vec!["@i".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.jump().is_none());

        let lines = vec!["M=1".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.jump().is_none());

        let lines = vec!["0;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.jump().is_some());
        assert_eq!(p.jump().unwrap(), "JMP".parse::<String>().unwrap());

        let lines = vec!["D+1;JMP".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.jump().is_some());
        assert_eq!(p.jump().unwrap(), "JMP".parse::<String>().unwrap());

        let lines = vec!["(LOOP)".to_string()];
        let p = Parser::new(&lines).ok().unwrap();
        assert!(p.jump().is_none());
    }
}