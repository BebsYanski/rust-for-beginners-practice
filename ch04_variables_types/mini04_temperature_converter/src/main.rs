use std::io;

fn main() {
    let mut input = String::new();
    println!("Give a temperature: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    // let temp: f64 = input.trim().parse().expect("Not a number");
    match input.trim().parse::<f64>() {
        Ok(value) => {
            cel_to_fahr(value);
            fahr_to_cel(value);
        }
        Err(error) => println!("You did not input a valid float\nError: {}", error),
    }
}

fn cel_to_fahr(temp: f64) {
    let fahr = temp * (9_f64 / 5_f64) + 32_f64;

    println!("{}℃ = {}℉", temp, fahr);
}

fn fahr_to_cel(temp: f64) {
    let cel = (temp - 32_f64) * (5_f64 / 9_f64);

    println!("{}℉ = {}℃", temp, cel);
}
