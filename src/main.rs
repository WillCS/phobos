use std::fs;

mod tokenisation;

fn main() {
    let contents = fs::read_to_string("input/test.lua")
        .expect("Something bad happened");

    tokenisation::tokenise(&contents);
}