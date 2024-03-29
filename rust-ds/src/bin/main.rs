use dslib::{self, queue::QueueTrait, stack::StackTrait};

fn main() {
    let mut stk: dslib::Stack<i16> = dslib::Stack::<i16>::new();
    stk.push(1);
    print!("{}", stk.pop().unwrap_or_else(|| -1));

    let mut sq: dslib::Simple<i16> = dslib::Simple::<i16>::new();
    sq.enqueue(1);
    print!("{}", sq.dequeue().unwrap_or_else(|| -1));
}
