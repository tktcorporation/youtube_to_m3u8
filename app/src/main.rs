use curl::easy::Easy;
use std::io::{stdout, Write, stdin};

fn main() {
    let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
