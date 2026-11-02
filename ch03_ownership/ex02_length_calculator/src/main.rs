fn main() {
    // s is borrowed, so the value of s remains since ownership does not change.
    let s = String::from("Yannick");
    let s_length = calculate_length(&s);

    println!("S: {}\nLength of s: {}", s, s_length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
