mod read_lines;
mod select_boolean_in_console;
use passwords::PasswordGenerator;
use read_lines::read_lines;
use select_boolean_in_console::select_boolean_in_console;
fn main() {
    let is_length: usize = read_lines("Escreva o tamanho da password: ".to_string())
        .trim()
        .parse()
        .unwrap();

    let is_numbers: bool =
        select_boolean_in_console(read_lines("Possui números? (S/N): ".to_string()));
    let is_lowercase_letters: bool =
        select_boolean_in_console(read_lines("Possui letras minusculas? (S/N): ".to_string()));
    let is_uppercase_letters: bool =
        select_boolean_in_console(read_lines("Possui letras maiusculas? (S/N): ".to_string()));
    let is_symbols: bool =
        select_boolean_in_console(read_lines("Possui símbolos? (S/N): ".to_string()));
    let is_spaces: bool =
        select_boolean_in_console(read_lines("Possui espaços? (S/N): ".to_string()));
    let is_exclude_similar_characters: bool =
        select_boolean_in_console(read_lines("Possui caracteres iguais? (S/N): ".to_string()));
    let is_strict: bool = select_boolean_in_console(read_lines("Senha forte? (S/N): ".to_string()));
    let amount: usize = read_lines("Quantidade de passwords: ".to_string())
        .trim()
        .parse()
        .unwrap();

    let password_options = PasswordGenerator {
        length: is_length,
        numbers: is_numbers,
        lowercase_letters: is_lowercase_letters,
        uppercase_letters: is_uppercase_letters,
        symbols: is_symbols,
        spaces: is_spaces,
        exclude_similar_characters: is_exclude_similar_characters,
        strict: is_strict,
    };

    let password_iterator = password_options.generate(amount).unwrap();

    for password in &password_iterator {
        println!("Password: {}", password)
    }

    println!("Passwords geradas com sucesso!!");
}
