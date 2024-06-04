use std::io::{self, Write};
use std::process::Command;

fn main() {
    // println!("EnigmaSH Shell prototype");
    let mut input = String::new();
    loop{
        print!("EnigmaSH> ");
        io::stdout().flush().expect("failed to push STDOUT");
        if let Ok(bytes_read) = io::stdin().read_line(&mut input) {
            if bytes_read == 1 { // Empty line (only newline character)
                input.clear();
                continue;
            }
        } else {
            eprintln!("Failed to read input");
            break;
        }
        let mut tokens = input.trim().split_whitespace();
        let command = tokens.next().expect("No Command Found");
        // let mut args = tokens.collect::<Vec<_>>();
        let args = tokens.collect::<Vec<_>>();

        // // Print command and arguments
        // println!("Command: {}", command);
        // println!("Arguments: {:?}", args);

        match command {
            "help" => {
                println!("Built-in commands:");
                println!("  help  - Display this help message");
                println!("  exit  - Exit the EnigmaSH shell");
                println!("  clear  - Clear the terminal screen");
            },
            "exit" => {
                println!("Exiting EnigmaSH...");
                break;
            },
            "clear" => {
                /*
                \x1B is the escape character.
                [2J clears the entire screen.
                [1;1H moves the cursor to the top-left corner.
                */
                print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top-left
                io::stdout().flush().expect("Failed to flush STDOUT");
            },
            _ => {
                let output = Command::new(command)
                    .args(&args)
                    .output();

                match output {
                    Ok(output) => {
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            print!("{stdout}");
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            eprintln!("Error: {stderr}");
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to execute command: {}", e);
                    }
                }
            },
        }
        input.clear();
    }
}
