use std::fs;

mod tokenisation;

fn main() {
    let contents = fs::read_to_string("input/test.lua")
        .expect("Something bad happened");

    let mut tokeniser = tokenisation::Tokeniser::new(&contents);

    // println!("{}", tokeniser.has_next_char());
    // while tokeniser.has_next_char() {
    //     println!("{} at line {}, col {}", tokeniser.get_next_char().unwrap(), tokeniser.location.line, tokeniser.location.col)
    // }

    let tokens = tokeniser.tokenise();

    for token in tokens {
        println!("{}", token);
    }
}