use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    // This flush ensures the prompt is printed before waiting for input
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
