// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

#[macro_use]
mod my_macro {
    macro_rules! say_hello {
        ($a:expr) => {
            concat!("Hello ", $a)
        };
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(say_hello!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(say_hello!("goodbye!"), "Hello goodbye!");
    }
}
