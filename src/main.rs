use std::io;

fn binary_translator(input: String) -> String {
    let output = input.bytes()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join(" ");
    output
}

fn main() {
    loop {
        let mut input_text = String::new();
        println!("Ingrese su texto a traducir (o 'salir' para terminar):");
        io::stdin().read_line(&mut input_text).unwrap();
        input_text = input_text.trim().to_string();

        if input_text.to_lowercase() == "salir" {
            break;
        }

        let translated_value = binary_translator(input_text);
        println!("Texto traducido: {}\n", translated_value);
    }

    println!("Gracias por usar el traductor!");
}