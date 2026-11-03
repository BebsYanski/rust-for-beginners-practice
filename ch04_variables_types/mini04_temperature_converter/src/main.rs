use std::io;

fn main() {
    let mut input = String::new();
    println!("Give a temperature: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    // let temp: f64 = input.trim().parse().expect("Not a number");
    // match input.trim().parse::<f64>() {
    //     Ok(value) => {
    //         cel_to_fahr(value);
    //         fahr_to_cel(value);
    //     }
    //     Err(error) => println!("You did not input a valid float\nError: {}", error),
    // }

    let temp: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(error) => {
            println!("There was an error - {}", error);
            return;
        }
    };

    let mut scale = String::new();
    println!("Enter scale (C/F): ");
    io::stdin()
        .read_line(&mut scale)
        .expect("could not read scale.");

    match scale.trim().to_lowercase().as_str() {
        "c" | "celsius" => cel_to_fahr(temp),
        "f" | "fahrenheit" => fahr_to_cel(temp),
        _ => println!("Unrecognized scale"),
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
