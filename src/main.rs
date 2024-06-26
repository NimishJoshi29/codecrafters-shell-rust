use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    loop {
        let mut input = String::new();
        print_prompt_and_get_input(&mut input);
        if input == "exit 0" {
            exit(0);
        }
        handle_unknown_command(&input);
    }
}

fn print_prompt_and_get_input(command: &mut String) {
    print!("$ ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let _ = stdin.read_line(command);
    command.pop();
}

fn handle_unknown_command(input: &str) {
    println!("{}: command not found", input);
}
