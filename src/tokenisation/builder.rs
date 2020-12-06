use std::collections::{HashMap, HashSet};
use crate::tokenisation::tokeniser::Tokeniser;

pub struct TokeniserBuilder<'t, T> {
    static_tokens:               HashMap<String, T>,
    static_tokens_with_fallback: HashMap<String, (T, &'t dyn FnMut(String) -> T)>,
    dynamic_tokens:              HashMap<char, &'t dyn FnMut(String) -> T>
}

impl<'t, T> TokeniserBuilder<'t, T> {
    pub fn new<'u, U>() -> TokeniserBuilder<'u, U> {
        TokeniserBuilder::<'u, U> {
            static_tokens:               HashMap::new(),
            static_tokens_with_fallback: HashMap::new(),
            dynamic_tokens:              HashMap::new()
        }
    }

    pub fn with_static_token(mut self,
        token_str:  &'t str,
        token_type: T
    ) -> TokeniserBuilder<'t, T> {
        self.static_tokens.insert(String::from(token_str).clone(), token_type);
        return self;
    }

    pub fn with_static_token_with_fallback(mut self,
        token_str:   &'t str,
        token_type:  T,
        fallback_fn: &'t dyn FnMut(String) -> T
    ) -> TokeniserBuilder<'t, T> {
        self.static_tokens_with_fallback.insert(String::from(token_str).clone(), (token_type, fallback_fn));
        return self;
    }

    pub fn with_dynamic_token(mut self,
        start_chars: HashSet<char>,
        parse_fn:    &'t dyn FnMut(String) -> T
    ) -> TokeniserBuilder<'t, T> {
        for c in start_chars {
            self.dynamic_tokens.insert(c, parse_fn);

        }
        return self;
    }

    pub fn build(self) -> Result<Tokeniser<'t>, ()> {
        Err(())
    }
}
