mod tokenisation;
mod parsing;
mod lua;

pub use tokenisation::get_lua_tokeniser;
pub use parsing::get_lua_parser;

pub fn test_fn() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(test_fn())
    }
}
