use std::io;

fn main() {
    println!("start...");

    let mut guess = String::new(); // mut: 가변 변수

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}
