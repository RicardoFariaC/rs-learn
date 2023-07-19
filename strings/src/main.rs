use std::io;

fn main() {
    // string reference
    let nome_str: &str = "ricardo";
    // [inicio, len]
    println!("{nome_str}");
    
    // heap string
    let mut nome_str = String::new();
    nome_str.push_str("ricardo");
    println!("{nome_str}");

    let _nome_str: String = "ricardo".to_string();

    let _nome_str = String::from("ricardo");

    // rust io

    let mut input_string = String::new();
    println!("Digite no terminal: ");

    io::stdin()
        .read_line(&mut input_string)
        .expect("Error reading console.");

    println!("Você digitou: {input_string}");


}
