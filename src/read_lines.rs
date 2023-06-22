use std::io::{self, Write};

pub fn read_lines(msg: String) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    let _ = io::stdin().read_line(&mut input);

    return input;
}
