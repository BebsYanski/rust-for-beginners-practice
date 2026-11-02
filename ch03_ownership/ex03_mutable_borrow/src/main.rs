fn main() {
    // s must be mutable in order for its value to change
    let mut s = String::from("Yan");
    let s1 = &mut s;

    // add_exclamation(&mut s);
    // add_exclamation(&mut s);

    s1.push_str("string");

    println!("{}", s);
}

fn add_exclamation(s: &mut String) {
    s.push_str("!");
}
