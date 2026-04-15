use std::io::{self, BufRead, Write};

fn main() {
    println!("SI Unit Converter");
    println!("Type 'exit' to quit");
    println!();

    let stdin = io::stdin();

    loop {
        print!("calculate: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!("Input: {}", input);
    }
}
