fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let fahrenheit = convert_to_fahrenheit(30.0);
    println!("30 degrees Celsius is equal to {fahrenheit} degrees Fahrenheit.");
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}
