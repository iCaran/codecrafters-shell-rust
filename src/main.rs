#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read the user input
        let mut input = String::new();
        let bytes_read = stdin.read_line(&mut input).unwrap();

        // If bytes_read is 0, it means EOF (Ctrl+D), break the loop
        if bytes_read == 0 {
            break;
        }

        // Trim the input to remove any extra whitespace
        let command = input.trim();

        // Check if the command is empty or unrecognized
        if command.is_empty() {
            continue; // Skip empty input
        } else {
            // Print the error message for an unrecognized command
            println!("{}: command not found", command);
        }
    }
}
