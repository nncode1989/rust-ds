use dslib::stack::Stack;

fn main() {
    println!("{}",dslib::welcome_message());
    let mut st: Stack<i16> = Stack::new(Some(3));
    st.push(1);
}