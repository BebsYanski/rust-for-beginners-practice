fn main() {
    const MAX_SCORE: u32 = 100;
    let mut score = 0;

    loop {
        println!("{score}");
        score += 1;
        if score == MAX_SCORE {
            break;
        }
    }
}
