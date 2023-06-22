use std::io::{self, Write};

use passwords::PasswordGenerator;

fn select_boolean_in_console(option: String) -> bool {
    if option == "N" || option == "n" {
        return false;
    }

    return true;
}

fn read_lines(msg: String) -> String {
    let mut input: String = String::new();
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut input);
    print!("{msg}");

    return input;
}

fn main() {
    let mut is_numbers: bool = false;
    let mut is_lowercase_letters: bool = true;
    let mut is_uppercase_letters: bool = false;
    let mut is_symbols: bool = false;
    let mut is_spaces: bool = false;
    let mut is_exclude_similar_characters: bool = false;
    let mut is_strict: bool = false;

    let mut amount: usize = 1;

    let is_length: usize = read_lines("Escreva o tamanho da password: ".to_string())
        .trim()
        .parse()
        .unwrap();

    println!("{is_length}");

    let pg = PasswordGenerator {
        length: is_length,
        numbers: is_numbers,
        lowercase_letters: is_lowercase_letters,
        uppercase_letters: is_uppercase_letters,
        symbols: is_symbols,
        spaces: is_spaces,
        exclude_similar_characters: is_exclude_similar_characters,
        strict: is_strict,
    };

    println!("{}", pg.generate_one().unwrap());
    println!("{:?}", pg.generate(amount).unwrap());
}
