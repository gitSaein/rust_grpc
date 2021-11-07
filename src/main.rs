use rand::Rng;

fn main() {
    println!("guess number");
    let mut rng = rand::thread_rng();

    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");

    println!("rand number: {}", rng.gen_range(0..10));

}
