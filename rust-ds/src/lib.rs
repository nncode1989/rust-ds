#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}

pub use self::stack::Stack;

/// Adds one to the number given.
pub mod stack;