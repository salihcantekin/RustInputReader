#![allow(unused_imports, dead_code)]

mod input_values;
mod validators;
mod input_readers;
mod user_io;
mod converters;

use std::io::{self, Read, Write};
use std::str::FromStr;
use crossterm::{execute, style::{Color, PrintStyledContent, Stylize}, terminal};
use input_readers::bool_input_reader::BoolInputReader;
use input_readers::generic_input_reader::{GenericInputReader, InputReaderExtensions};
use input_readers::int_input_reader::IntInputReader;
use input_values::input_value::InputValue;
use input_values::int_input_value::IntInputExtensions;
use validators::length_validator::LengthValidator;
use validators::validator::*;


pub struct Input;

impl Input {
    fn int() -> IntInputReader {
        IntInputReader::new()
    }

    fn bool() -> BoolInputReader {
        BoolInputReader::new()
    }
}

fn main() {
    let int_input_value = GenericInputReader::<i32>::new() //Input::int()
        .with_message("Please enter a number: ")
        .with_error_message("Wrong number format! Try again.")
        .prevalidate(|input| {
            if input.len() < 5 {
                return false;
            }
            true
        })
        .read_until_valid();

    println!("Val: {:#?}", int_input_value);

    // GenericInputReader::<i32>::new()
    //     .with_message("Enter a number: ")
    //     .with_error_message("Invalid number!")
    //     .read_until_valid();
}

