use std::env;
use std::io;
mod encode;

fn handle_inputs_from_args(args: &[String]) -> (String, i32) {
    let key = args[2].parse::<i32>().expect("Invalid input for key");
    let text = if args.len() > 3 {
        args[3..].join(" ")
    } else {
        println!("Enter text: ");
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("Failed to read line");
        text.trim().to_string()
    };

    (text, key)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <encode|decode> <key> [message]", args[0]);
        return;
    }

    let operation = &args[1];
    let (text, key) = handle_inputs_from_args(&args);

    match operation.as_str() {
        "encode" => {
            let encoded_text = encode::encode(text, key);
            println!("Encoded text: {}", encoded_text);
        }
        "decode" => {
            let decoded_text = encode::encode(text, -key);
            println!("Decoded text: {}", decoded_text);
        }
        _ => {
            println!("Unknown operation: {}", operation);
            println!("Usage: {} <encode|decode> <key> [message]", args[0]);
        }
    }
}
