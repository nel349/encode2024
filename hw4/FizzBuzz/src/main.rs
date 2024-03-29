fn main() {
    println!("Hello, world!");
    fizz_buzz();
}

fn fizz_buzz() {
    for n in 1..=301 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("fizz buzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        }
    }
}
