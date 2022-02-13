#![warn(clippy::all, clippy::pedantic)]

fn main() {
    println!("Hello, world!");
    clippy();
}

fn clippy() {
    let my_list = [ "One", "Two", "Three"];
    for i in &my_list {
        println!("{}", i);
    }
}
