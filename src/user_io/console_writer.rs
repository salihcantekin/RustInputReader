use std::io::{self, Write};

use crossterm::{execute, style::{Color, PrintStyledContent, Stylize}};

use super::default_console::DefaultConsole;

pub trait ConsoleWriter {
    fn write(&self, message: &str);
    fn writeln(&self, message: &str);
    fn write_with_color(&self, message: &str, color: Color);
    fn writeln_with_color(&self, message: &str, color: Color);
}



impl ConsoleWriter for DefaultConsole {
    fn write(&self, message: &str) {
        print!("{}", message);
        io::stdout().flush().unwrap();
    }
    fn writeln(&self, message: &str) {
        println!("{}", message);
    }
    fn write_with_color(&self, message: &str, color: Color) {
        execute!(io::stdout(), PrintStyledContent(message.with(color))).unwrap();
    }
    fn writeln_with_color(&self, message: &str, color: Color) {
        execute!(io::stdout(), PrintStyledContent(message.with(color)), PrintStyledContent("\n".with(Color::Reset))).unwrap();
    }
}