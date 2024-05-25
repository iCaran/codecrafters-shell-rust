#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    // Trim the input to remove any extra whitespace
    let command = input.trim();

    // Check if the command is empty or unrecognized
    if command.is_empty() {
        // If the command is empty, you can choose to ignore or handle it accordingly
    } else {
        // Print the error message for an unrecognized command
        println!("{}: command not found", command);
    }
}
