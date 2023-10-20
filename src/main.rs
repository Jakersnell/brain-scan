use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// BrainFuck interpreter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///file to read
    filename: String,
}

// > = increases memory pointer, or moves the pointer to the right 1 block.
// < = decreases memory pointer, or moves the pointer to the left 1 block.
// + = increases value stored at the block pointed to by the memory pointer
// - = decreases value stored at the block pointed to by the memory pointer
// [ = like c while(cur_block_value != 0) loop.
// ] = if block currently pointed to's value is not zero, jump back to [
// , = like c getchar(). input 1 character.
// . = like c putchar(). print 1 character to the console

/// util for printing errors to terminal in red.
fn print_err<T: core::fmt::Display>(error_type: &str, error: T) {
    println!("{}", format!("{} {}", error_type, error).red());
}
// const and static variables
const MEM_SIZE: usize = 4000;
// static mut BYTES: [u8; MEM_SIZE] = [0; MEM_SIZE];

// file read and execution
fn main() {
    let args = Args::parse();

    if args.filename.is_empty() {
        println!("{}", "No file provided".red());
        return;
    }
    if args.filename.ends_with(".bf") == false {
        println!("{}", "Invalid file type".red());
        return;
    }
    match File::open(&args.filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();

            if let Err(error) = reader.read_to_string(&mut contents) {
                print_err("error in processing file: ", error);
            } else {
                let mut interpreter = Interpreter::new(contents.chars().collect());
                interpreter
                    .execute()
                    .unwrap_or_else(|error| print_err("execution error: ", error));
            }
        }
        Err(error) => {
            print_err("error in reading file: ", error);
        }
    }
}

struct Interpreter {
    mem: [u8; MEM_SIZE],
    current_value: u8,
    pointer: usize,
    index: usize,
    tokens: Vec<char>,
    loop_starts: Vec<usize>,
}
impl Interpreter {
    // Lifecycle methods
    pub fn new(tokens: Vec<char>) -> Interpreter {
        Interpreter {
            mem: [0; MEM_SIZE],
            current_value: 0,
            pointer: 0,
            index: 0,
            tokens: tokens,
            loop_starts: Vec::new(),
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let mut token;
        loop {
            token = self.tokens[self.index];

            match token {
                '\n' | '\t' | ' ' => {}

                '>' => self.increment_pointer(),
                '<' => self.decrement_pointer(),
                '+' => self.increment_value(),
                '-' => self.decrement_value(),
                ',' => self.input_char()?,
                '.' => self.output_char(),
                '[' => self.push_loop_marker(),
                ']' => {
                    if self.mem[self.pointer] == 0 {
                        self.pop_loop_marker();
                    } else {
                        self.jumpto_last_marker()?;
                    }
                }

                _ => return Err(format!("Invalid token at line: {}", self.index)),
            }
            self.index += 1;
            if self.index >= self.tokens.len() {
                break;
            }
        }
        println!();
        Ok(())
    }

    // Pointer manipulation methods
    pub fn increment_pointer(&mut self) {
        self.pointer = self.pointer.wrapping_add(1);
        self.pointer %= MEM_SIZE;
        self.current_value = self.mem[self.pointer];
    }

    pub fn decrement_pointer(&mut self) {
        self.pointer = self.pointer.wrapping_sub(1);
        self.pointer %= MEM_SIZE;
        self.current_value = self.mem[self.pointer];
    }

    // Value manipulation methods
    pub fn increment_value(&mut self) {
        self.mem[self.pointer] = self.mem[self.pointer].wrapping_add(1);
    }

    pub fn decrement_value(&mut self) {
        self.mem[self.pointer] = self.mem[self.pointer].wrapping_sub(1);
    }

    pub fn get_current_value(&mut self) -> u8 {
        self.mem[self.pointer]
    }

    pub fn input_current_value(&mut self, value: u8) {
        self.mem[self.pointer] = value;
    }

    // Loop handling methods
    pub fn push_loop_marker(&mut self) {
        self.loop_starts.push(self.index);
    }

    pub fn pop_loop_marker(&mut self) {
        self.loop_starts.pop();
    }

    pub fn jumpto_last_marker(&mut self) -> Result<(), &'static str> {
        if self.loop_starts.is_empty() {
            Err("No marker tokens in tokens. null loop jump.")
        } else {
            self.index = *self.loop_starts.last().unwrap();
            Ok(())
        }
    }

    // I/O methods
    pub fn output_char(&mut self) {
        print!("{}", self.get_current_value() as char);
    }

    pub fn input_char(&mut self) -> Result<(), &str> {
        let mut input = String::new();
        if let std::io::Result::Err(_) = std::io::stdin().read_line(&mut input) {
            return Err("Error in reading input");
        }
        self.input_current_value(input.as_bytes()[0]);
        Ok(())
    }
}
