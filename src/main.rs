use std::fs::{File, OpenOptions};
use std::io::{self, Error, Write};
use std::process::{Command, Stdio};

fn get_user_input() -> Result<String, io::Error> {
    print!("EnigmaSH> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_input(input: &str) -> (String, Vec<String>, Option<String>, Option<String>, bool) {
    let tokens: Vec<_> = input.trim().split_whitespace().collect();
    let mut command_args = Vec::new();
    let mut output_file: Option<String> = None;
    let mut input_file: Option<String> = None;
    let mut append_mode = false;
    // time to seprate arguments based on their types (I/O file arg or just cmd args)
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            ">" => {
                if i + 1 < tokens.len() {
                    output_file = Some(tokens[i + 1].to_string());
                    i += 1;
                }
            },
            ">>" => {
                if i + 1 < tokens.len() {
                    output_file = Some(tokens[i + 1].to_string());
                    append_mode = true;
                    i += 1;
                }
            },
            "<" => {
                if i + 1 < tokens.len() {
                    input_file = Some(tokens[i + 1].to_string());
                    i += 1;
                }
            },
            _ => command_args.push(tokens[i].to_string()),
        }
        i += 1;
    }
    // let command = tokens.first().expect("No Command Found").to_string(); // Use first()
    // (command, command_args, output_file, input_file)
    let command = command_args.first().cloned().unwrap_or_else(|| "".to_string());      //guessing that in case cmd not found would be run by "" output
    (command, command_args, output_file, input_file, append_mode)
}

fn handle_builtin_command(command: &str) {
    match command {
        "help" => println!("... (help message)"),
        "exit" => println!("Exiting EnigmaSH..."),
        "clear" => print!("\x1B[2J\x1B[1;1H"),
        _ => println!("Unknown command. Type 'help' for a list of available commands."),
    }
}

fn handle_external_command(
    command: &str,
    args: &[String],
    output_file: Option<String>,
    input_file: Option<String>,
    append_mode: bool,
) -> Result<(), io::Error> {

    let mut command = Command::new(command);
    command.args(&args[1..]);
    
    // I/O file handling
    if let Some(filename) = output_file {
        let output_file = if append_mode {
            OpenOptions::new().create(true).append(true).open(filename)?
        } else {
            File::create(filename)?
        };
        command.stdout(Stdio::from(output_file));
    } else if let Some(filename) = input_file {
        let input_file = File::open(filename)?;
        command.stdin(Stdio::from(input_file));
    }

    let resultant = command.output()?;
    if resultant.status.success() {
        let stdout = String::from_utf8_lossy(&resultant.stdout);
        println!("{stdout}");
    } else {
        let stderr = String::from_utf8_lossy(&resultant.stderr);
        eprintln!("Error: {stderr}");
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    loop {
        // Get user input
        let input = get_user_input()?;

        // Parse input with redirection information
        let (command, args, output_file, input_file, append_mode) = parse_input(&input);

        // Handle exit command
        if command == "exit" {
            println!("Exiting EnigmaSH...");
            break;
        }

        // Handle built-in commands
        match command.as_str() {
            "help" | "exit" | "clear" => handle_builtin_command(&command),
            "" => (), // Skip empty input
            _ => {
                // Handle external commands with potential redirection
                match handle_external_command(&command, &args, output_file, input_file, append_mode) {
                    Ok(_) => (), // Do nothing on successful execution
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        }
    }
    Ok(())
}
