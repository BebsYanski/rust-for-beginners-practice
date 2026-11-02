fn main() {
    let s1 = "This is a great day to be happy";
    let s2 = String::from(s1);

    println!("With the String Literal");
    println!("char_count - word_count: {:?}", stats(s1));

    println!("With the String");
    println!("char_count - word_count: {:?}", stats(&s2));
}

fn stats(s: &str) -> (usize, usize) {
    let character_count = s.chars().count();
    let word_count = s.split_whitespace().count();

    (character_count, word_count)
}
