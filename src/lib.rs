mod tokenisation;
mod parsing;
mod lua;

pub use lua::{get_lua_tokeniser, get_lua_parser};

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
