use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    loop {
        let mut input = String::new();
        print_prompt_and_get_input(&mut input);
        match input.split_whitespace().next() {
            Some("exit") => exit(0),
            Some("echo") => handle_echo_command(&input),
            Some("type") => handle_type_command(&input),
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

fn handle_type_command(input: &str) {
    let mut itr = input.split_whitespace();
    itr.next();
    let command = itr.next().unwrap();
    let builtins = ["echo", "exit", "type"];
    if builtins.contains(&command) {
        println!("{} is a shell builtin", command);
    } else {
        println!("{}: not found", command);
    }
}
