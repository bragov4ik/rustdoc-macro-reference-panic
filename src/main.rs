//! Some module with [macro@some_cool_macro!]

#[macro_export]
macro_rules! some_cool_macro {
    () => {};
}

fn main() {
    println!("Hello, world!");
}
