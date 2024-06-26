use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    loop {
        let mut input = String::new();
        print_prompt_and_get_input(&mut input);
        match input.as_str() {
            "exit 0" => exit(0),
            "exit" => exit(0),
            s if s.starts_with("echo ") => handle_echo_command(&input),
            _ => handle_unknown_command(&input),
        }
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

fn handle_echo_command(command: &str) {
    for (i, c) in command.char_indices() {
        if i < 5 {
            continue;
        }
        print!("{}", c);
    }
    println!("");
}
