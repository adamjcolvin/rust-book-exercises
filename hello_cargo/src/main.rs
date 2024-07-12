use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Guess the number between 1 and 100!");
    println!("The secret number is: {}", secret_number);
}
