#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}

pub use self::stack::Stack;
pub use self::queue::simple::Simple;
/// Adds one to the number given.
pub mod stack;
pub mod queue;