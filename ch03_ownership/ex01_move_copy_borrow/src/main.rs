fn main() {
    // string is moved entirely; hence s1 no longer exists after the move.
    let s1 = String::from("Yannick");
    let s2 = s1;

    // i32 implements the copy trait, so the value of my_num1 is copied
    let my_num1: i32 = 23;
    let my_num2: i32 = my_num1;

    // str2 borrows str1 using the reference to str1
    let str1 = String::from("Hello Yan");
    let str2 = &str1; // Has a reference to str1 for viewing only

    println!("str1: {}, str2: {}", str1, str2);
}
