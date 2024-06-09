use std::io::{self};
fn main() {
    let a = 10;
    let b = 24;
    println!("Hello, world!, {} {}", a, b);
    asd();
}

fn asd() {
    let mut guess: String = String::new();
    // let mut x;
    // io::stdin().read(&mut x);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // let a: &str;
    // let ad = 123;
    // a = "hehe";
    print!("pranjalverma {}, {}", guess, guess)
}
