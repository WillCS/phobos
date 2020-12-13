mod tokenisation;
mod parsing;
mod syntax_tree;

pub use tokenisation::get_lua_tokeniser;

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
