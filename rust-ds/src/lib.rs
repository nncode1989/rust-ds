#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn welcome_message() -> &'static str{
    "Hello Welcone to Rust DataStructure lessons"
}

pub mod stack;
pub mod cqueue;
