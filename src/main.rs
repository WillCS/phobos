use std::fs;
use phobos::get_lua_tokeniser;

fn main() {
    let contents = fs::read_to_string("input/test.lua")
        .expect("Something bad happened");

    let tokeniser = get_lua_tokeniser();
    if tokeniser.is_some() {
        let mut t = tokeniser.unwrap();
        
        let tokens = t.tokenise(contents);

        if tokens.is_err() {
            panic!("{:?}", tokens.err());
        } else {
            for token in tokens.unwrap() {
                println!("{:?}", token);
            };
        }
    }
}