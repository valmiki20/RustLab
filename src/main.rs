//use rand::Rng;
//use std::cmp::Ordering;
//use std::io;
/*
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess :u32 = guess.trim().parse().expect("please type a number");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }


}

 */
fn main() {
    let mut s = String::from("Hello World!!!!");
    println!("{}",s);
    s.push_str("from Nandiesh");
    println!("{}",s)
}