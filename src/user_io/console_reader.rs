use std::io::{self, Read};

use super::default_console::DefaultConsole;

pub(crate) trait ConsoleReader {
    fn read_line(&self) -> String;
    fn read_key(&self) -> char;
}

impl ConsoleReader for DefaultConsole {
    fn read_line(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    
    fn read_key(&self) -> char {
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0] as char
    }
}