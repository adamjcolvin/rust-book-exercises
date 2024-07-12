fn main() {
    let fibonaci_number = fibonaci(10);

    println!("The 10th number in the Fibonaci sequence is {fibonaci_number}.");
}

fn fibonaci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonaci(n - 1) + fibonaci(n - 2);
    }
}
