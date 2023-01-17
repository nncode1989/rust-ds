use dslib::{self, stack::StackTrait};

fn main() {
    let mut stk : dslib::Stack<i16> = dslib::Stack::<i16>::new();
    stk.push(1);
    print!("{}",stk.pop().unwrap_or_else(|| -1));
}