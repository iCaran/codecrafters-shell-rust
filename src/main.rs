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
        } else if command=="exit 0" {
            break;
        } else if command.starts_with("type ") {
            let type_args = &command[5..]; // Get the arguments after "echo "
            if type_args=="type" || type_args=="echo" || type_args=="exit" || type_args=="cat" {
                println!("{} is a shell builtin", type_args);
            } else {
                println!("{} not found", type_args);
            }
        } else if command.starts_with("echo ") {
            // Handle the echo command
            let echo_args = &command[5..]; // Get the arguments after "echo "
            println!("{}", echo_args);
        } else {
            // Print the error message for an unrecognized command
            println!("{}: command not found", command);
        }
    }
}
