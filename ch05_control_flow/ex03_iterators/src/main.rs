fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<&i32> = nums.iter().filter(|&x| x % 2 == 0).collect();
    let sq_evens: Vec<i32> = nums
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * *x)
        .collect();
    let sq_sum: i32 = nums.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum();

    println!("The Evens:\n{:?}", evens);
    println!("The sq_evens:\n{:?}", sq_evens);
    println!("The sum of all Squares:\n{}", sq_sum);
}
