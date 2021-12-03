use std::io;

fn main() {
    println!("game start!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("guess is {}", guess)
}
