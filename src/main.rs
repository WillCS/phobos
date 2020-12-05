use std::fs;

mod tokenisation;

fn main() {
    let contents = fs::read_to_string("input/test.lua")
        .expect("Something bad happened");

    let mut tokeniser = tokenisation::Tokeniser::new(&contents);

    match tokeniser.tokenise() {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token)
            }
        },
        Err(error) => {
            println!("{:?}", error)
        }
    }
}