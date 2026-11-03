/// FizzBuzz Challenge

fn main() {
    for n in 1..=30 {
        match n {
            n if n % 3 == 0 && n % 5 == 0 => println!("FizzBuzz"),
            n if n % 5 == 0 => println!("Buzz"),
            n if n % 3 == 0 => println!("Fizz"),
            _ => println!("{}", n),
        }
    }
}
