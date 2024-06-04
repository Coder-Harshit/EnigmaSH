use std::fs::File;
use std::io::{self, Error, Write};
use std::process::Command;

fn get_user_input() -> Result<String, io::Error> {
    print!("EnigmaSH> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_input(input: &str) -> (String, Vec<String>, Option<String>, Option<String>) {
    let tokens: Vec<_> = input.trim().split_whitespace().collect();
    let mut command_args = Vec::new();
    let mut output_file: Option<String> = None;
    let mut input_file: Option<String> = None;
    for (i, arg) in tokens.iter().enumerate() {
        if *arg == ">" {
            output_file = Some(tokens[i + 1].to_string()); // Use indexing and to_string()
        } else if *arg == ">>" {
            output_file = Some(tokens[i + 1].to_string());
        } else if *arg == "<" {
            input_file = Some(tokens[i + 1].to_string());
        } else {
            command_args.push(arg.to_string());
        }
    }
    let command = tokens.first().expect("No Command Found").to_string(); // Use first()
    (command, command_args, output_file, input_file)
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
) -> Result<(), io::Error> {

    let mut command = Command::new(command);
    command.args(args);
    if let Some(filename) = output_file {
        let output_file = File::create(filename)?;
        command.stdout(output_file);
    } else if let Some(filename) = input_file {
        let input_file = File::open(filename)?;
        command.stdin(input_file);
    }

    let output = command.output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{stdout}");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {stderr}");
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    loop {
        // Get user input
        let input = get_user_input()?;

        // Parse input with redirection information
        let (command, args, output_file, input_file) = parse_input(&input);

        // Handle exit command
        if command == "exit" {
            println!("Exiting EnigmaSH...");
            break;
        }

        // Handle built-in commands
        match command.as_str() {
            "help" | "exit" | "clear" => handle_builtin_command(&command),
            _ => {
                // Handle external commands with potential redirection
                match handle_external_command(&command, &args, output_file, input_file) {
                    Ok(_) => (), // Do nothing on successful execution
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        }
    }
    Ok(())
}
