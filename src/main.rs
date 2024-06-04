use std::io::{self, Write};

fn main() {
    // println!("EnigmaSH Shell prototype");
    let mut input = String::new();
    loop{
        print!("EnigmaSH> ");
        io::stdout().flush().expect("failed to push STDOUT");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        // println!("OUT==>{input}");
        input.clear();
    }
}
