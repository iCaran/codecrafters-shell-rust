#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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
        } else if command=="exit 0"{
            break;
        } else if command.starts_with("echo ") {
            // Handle the echo command
            let echo_args = &command[5..]; // Get the arguments after "echo "
            println!("{}", echo_args);
        } else if command.starts_with("type ") {
            // Handle the type command
            let type_args = &command[5..]; // Get the arguments after "type "
            if type_args=="echo" || type_args=="exit" || type_args=="type"{
                println!("{} is a shell builtin", type_args);
            }
            else if let Some(executable_path) = find_executable(type_args) {
                println!("{} is {}", type_args, executable_path);
            } else {
                println!("{} not found", type_args);
            }
        } else {
            // Handle external commands
            let mut parts = command.split_whitespace();
            if let Some(program) = parts.next() {
                let args: Vec<&str> = parts.collect();
                if let Some(executable_path) = find_executable(program) {
                    match Command::new(executable_path)
                        .args(&args)
                        .output() {
                        Ok(output) => {
                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        },
                        Err(e) => {
                            eprintln!("Failed to execute {}: {}", program, e);
                        },
                    }
                } else {
                    println!("{}: command not found", program);
                }
            }
        }
    }
}

fn find_executable(command: &str) -> Option<String> {
    // Get the PATH environment variable
    if let Ok(path_var) = env::var("PATH") {
        // Split the PATH variable into individual directories
        let paths = path_var.split(':');
        // Iterate through each directory in PATH
        for path in paths {
            // Create the full path to the executable
            let executable_path = Path::new(path).join(command);
            // Check if the executable exists and is a file
            if executable_path.is_file() && fs::metadata(&executable_path).map(|m| m.permissions().readonly() == false).unwrap_or(false) {
                return Some(executable_path.to_string_lossy().to_string());
            }
        }
    }
    None
}
