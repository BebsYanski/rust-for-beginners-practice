fn main() {
    // loop
    let mut counter: i32 = 0;
    loop {
        let sq = counter.pow(2);
        if sq > 100 {
            break;
        }
        println!("{}", sq);
        counter += 1;
    }

    // while
    println!("\nWhile loop");
    counter = 0;
    while counter <= 10 {
        println!("{}", counter.pow(2));
        counter += 1;
    }

    // for — range expresses intent directly
    println!("\nFor Loop");
    for n in 0..=10 {
        println!("{}", n * n);
    }
}
