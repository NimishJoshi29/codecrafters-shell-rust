use std::{
    env, fs,
    io::{self, Write},
    process::{exit, Command},
};

fn main() -> ! {
    loop {
        let mut input = String::new();
        print_prompt_and_get_input(&mut input);
        let mut itr = input.split_whitespace();
        match itr.next() {
            Some("exit") => exit(0),
            Some("echo") => handle_echo_command(&input),
            Some("type") => handle_type_command(itr.next().expect("error")),
            some_command => {
                if let Some(path_to_exe) = get_path(some_command.expect("Error getting command.")) {
                    let mut output = Command::new(path_to_exe);
                    for arg in itr {
                        output.arg(arg);
                    }
                    let output = output.output().expect("Error executing the program.");
                    let mut prog_opt = String::new();
                    for opt in output.stdout {
                        prog_opt.push(opt as char);
                    }
                    print!("{}", prog_opt);
                } else {
                    handle_unknown_command(some_command.expect("Error unwraping"));
                }
            }
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
    println!();
}

fn handle_type_command(input: &str) {
    let builtins = ["echo", "exit", "type"];
    if builtins.contains(&input) {
        println!("{} is a shell builtin", input);
    } else {
        match get_path(input) {
            Some(p) => println!("{} is {}", input, p),
            None => println!("{}: not found", input),
        }
    }
}

fn get_path(command: &str) -> Option<String> {
    let path;
    match env::var("PATH") {
        Ok(r) => path = r,
        Err(e) => {
            println!("Error occured: {}", e);
            exit(0)
        }
    }
    for p in path.split(':') {
        let files = fs::read_dir(p).unwrap();
        for file in files {
            match file {
                Ok(f) => {
                    if match f.file_name().into_string() {
                        Ok(name) => name,
                        Err(_) => {
                            println!("Error converting file name");
                            exit(0);
                        }
                    } == command
                    {
                        return Some(
                            (match f.path().into_os_string().into_string() {
                                Ok(c) => c,
                                Err(_) => {
                                    println!("Error converting file name");
                                    exit(0);
                                }
                            })
                            .to_string(),
                        );
                    }
                }
                Err(e) => println!("Error while reading file of dir: {}", e),
            }
        }
    }
    None
}
