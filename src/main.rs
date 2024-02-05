use std::io;
mod encode;

fn handle_inputs() -> (String, i32) {
    println!("Enter text: ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");
    let text = text.trim().to_string();

    println!("Enter key: ");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read line");
    let key = key.trim().parse::<i32>().expect("Invalid input for key");

    (text, key)
}

fn main() {
    println!("Would you like to: \n1) Encode \n2) Decode");
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option = option.trim();

    if option == "1" {
      let (text, key) = handle_inputs();
      let encoded_text = encode::encode(text, key);
      println!("Encoded text: {}", encoded_text);
    } else if option == "2" {
      let (text, key) = handle_inputs();
      let key = key* -1;
      let encoded_text = encode::encode(text, key);
      println!("{}",encoded_text);
    } else {
      println!("Not understood");
    }
}

