use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    let _ = stdin.read_line(&mut input);
    input.pop();
    handle_unknown_command(&input);
}

fn handle_unknown_command(input: &str) {
    println!("{}: command not found", input);
}
