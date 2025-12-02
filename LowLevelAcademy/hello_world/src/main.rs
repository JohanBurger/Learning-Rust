use std::io;

fn main() {
    let mut my_string = String::new();

    io::stdin()
        .read_line(&mut my_string)
        .expect("Failed to read line");

    println!("You entered: {}", my_string.trim());
}
