# Rust Practice Plan Based on "Rust for Beginners" (Hernando Abella)

Below is a set of projects and exercises mapped to the book's chapters. I've ordered them by chapter as they appear in the table of contents, with difficulty and estimated time. Do them in order — later ones reuse earlier concepts.

---

## Chapter 1–2: Setup & Hello, World!
**Exercise 1.1 – Environment Check**
- Install Rust via `rustup`, then run `rustc --version` and `cargo --version`.
- Install `rustfmt` and `clippy` components.
- Create a `hello_world.rs`, compile with `rustc`, run it, then rebuild the same thing with `cargo new`.

**Exercise 1.2 – Multi-print**
- Write a program that prints your name, the Rust version you're using (hardcoded), and today's "goal" for learning Rust — each on its own line using one `println!` macro per line.

---

## Chapter 3: Understanding Ownership & Borrowing
**Exercise 3.1 – Ownership Detective**
- Write three small snippets: one that moves a `String`, one that copies an `i32`, one that borrows a `String`.
- For each, add a comment explaining *why* the code compiles or fails. Then deliberately break one (use a moved value) and read the compiler error carefully.

**Exercise 3.2 – Length Calculator**
- Write `fn calculate_length(s: &String) -> usize` and call it without transferring ownership. Print the original string afterward to prove it still lives.

**Exercise 3.3 – Mutable Borrow**
- Write `fn add_exclamation(s: &mut String)` and call it. Then try calling it twice on the same variable in one statement (e.g., `add_exclamation(&mut s); add_exclamation(&mut s);`) and observe that it's fine — then try holding two `&mut` at once and see the error.

**Mini-Project 3.4 – String Stats**
- Write a function `fn stats(s: &str) -> (usize, usize)` returning (character count, word count). Do not take ownership. Call it on a `String` and on a string literal.

---

## Chapter 4: Variables & Data Types
**Exercise 4.1 – Type Zoo**
- Declare one variable of each: `i8, i32, u64, f32, f64, bool, char`.
- Print each with `println!` using `{:?}` or `{}` as appropriate.

**Exercise 4.2 – Shadowing Puzzle**
- Start with `let x = 5;`, shadow it to a `&str`, then shadow again to an `f64`. Print at each step. Add comments on what type `x` is at each point.

**Exercise 4.3 – Constants & Mutability**
- Declare `const MAX_SCORE: u32 = 100;` and a `let mut score = 0;`. Increment score in a loop until it hits MAX_SCORE, breaking when done.

**Mini-Project 4.4 – Temperature Converter**
- Write a program that converts Celsius ↔ Fahrenheit. Use `f64`, take input via `std::io`, and handle invalid input (non-numeric) gracefully with a `match` on `parse()`.

---

## Chapter 5: Control Flow (if, match, loops, iterators)
**Exercise 5.1 – FizzBuzz (with a twist)**
- Classic FizzBuzz 1..=30, but use a `match` on `(n % 3, n % 5)` instead of `if/else`.

**Exercise 5.2 – Loop Control**
- Write a `loop` that increments a counter and breaks when the counter squared exceeds 100. Then rewrite it as a `while`. Then as a `for` over a range.

**Exercise 5.3 – Iterators**
- Given `let nums = vec![1,2,3,4,5,6,7,8,9,10];`, use `.iter().filter(...).map(...).sum()` to compute the sum of squares of even numbers.

**Mini-Project 5.4 – Guessing Game (text version)**
- Generate a "secret number" (hardcoded is fine), loop reading guesses from stdin, and use `match` on `(guess, secret)` to say "too low / too high / correct." Break on correct.

---

## Chapter 6: Pattern Matching
**Exercise 6.1 – Enum Matching**
- Define `enum Coin { Penny, Nickel, Dime, Quarter }` and write `fn value_in_cents(c: Coin) -> u8`. Call it for each variant.

**Exercise 6.2 – Destructuring**
- Define `struct Point { x: i32, y: i32 }` and write a `match` that classifies a point as "origin," "on x-axis," "on y-axis," or "general."

**Exercise 6.3 – Multiple Patterns & Guards**
- Write a function `fn classify(n: i32) -> &'static str` that uses `match` with `|`, ranges (`1..=9`), and a guard (`n if n < 0 => "negative"`).

**Mini-Project 6.4 – Shape Area Calculator**
- Define `enum Shape { Circle { radius: f64 }, Rectangle { w: f64, h: f64 }, Triangle { b: f64, h: f64 } }`. Write `fn area(s: &Shape) -> f64` using `match`.

---

## Chapter 7: Functions & Modules
**Exercise 7.1 – Function Library**
- Write four functions: `greet()`, `add(a,b)`, `print_message(&str)`, `max(a,b)`. Call all from `main`.

**Exercise 7.2 – Module Split**
- Create a `math` module with `pub fn add`, `pub fn sub`, `pub fn mul`. Call them from `main` using `math::add(...)`.

**Mini-Project 7.3 – Multi-file Crate**
- Turn 7.2 into a real multi-file project: `src/main.rs`, `src/math.rs`, use `mod math;` in main. Add a `pub` helper function inside `math`.

---

## Chapter 8: Error Handling (Result & Option)
**Exercise 8.1 – Divide Safely**
- Write `fn divide(a: i32, b: i32) -> Result<i32, &'static str>` returning `Err("Cannot divide by zero")` on `b == 0`. Test with both cases in `main` via `match`.

**Exercise 8.2 – Find Element**
- Write `fn find_element(arr: &[i32], target: i32) -> Option<usize>` using `iter().enumerate()` and the `&value` pattern (as in the book). Test with found and not-found cases.

**Exercise 8.3 – The ? Operator**
- Write a function that reads a number from a `&str`, parses it, and doubles it — returning `Result<i32, std::num::ParseIntError>` using `?`.

**Mini-Project 8.4 – File Reader**
- Write a program that reads a text file and prints its line count. Use `Result` and `?` for file-not-found handling. Print a friendly error on failure instead of panicking.

---

## Chapter 9: Structs & Enums
**Exercise 9.1 – Point Struct**
- Define `struct Point { x: f64, y: f64 }`, create an instance, and print its coordinates.

**Exercise 9.2 – Traffic Light**
- Define `enum TrafficLight { Red, Yellow, Green }` and a `match` that prints "Stop!" / "Prepare to stop!" / "Go!".

**Mini-Project 9.3 – Shape + Trait Combo**
- Define `enum Shape` as in 6.4, plus `impl Shape { fn area(&self) -> f64 }`. Then in `main`, create a `Vec<Shape>` and sum all areas.

---

## Chapter 10: Generics & Traits
**Exercise 10.1 – Generic Max**
- Write `fn find_max<T: PartialOrd>(a: T, b: T) -> T`. Test with integers and floats.

**Exercise 10.2 – Printable Trait**
- Define `trait Printable { fn print(&self); }`, implement for `Point`, call `origin.print()`.

**Mini-Project 10.3 – Shape Trait**
- Define `trait Shape { fn area(&self) -> f64; }`, implement for `Circle` and `Rectangle` structs. In `main`, put them in a `Vec<Box<dyn Shape>>` and sum their areas.

---

## Chapter 11: Concurrency
**Exercise 11.1 – Move into Thread**
- Spawn a thread with `thread::spawn(move || ...)` that owns a `Vec<i32>` and prints its contents. `join` in main.

**Exercise 11.2 – Channels**
- Create an `mpsc::channel()`, move the sender into a thread that sends a `String`, receive it in main and print it.

**Mini-Project 11.3 – Async Counter**
- Using `tokio` (add via Cargo), write an `async fn count_to(n: u32)` that `sleep`s 100 ms per tick and prints the number. `await` it from `#[tokio::main]`.

---

## Chapter 12: Advanced Topics (Lifetimes, Smart Pointers, Unsafe)
**Exercise 12.1 – Longest String**
- Write `fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str`. Test it. Then deliberately introduce a dangling-reference bug (assign result outside scope) and read the compiler error.

**Exercise 12.2 – Box**
- Create `let boxed = Box::new(42);`, print `*boxed` and the pointer value.

**Exercise 12.3 – Rc Tree**
- Build the `Node` tree from the book using `Rc<Node>` for children and `Rc::clone`. Print root and leaf data.

**Exercise 12.4 – Unsafe (educational only)**
- Write an `unsafe fn dangerous()` that prints a message, call it inside an `unsafe { }` block, and add a comment explaining *why* you'd avoid this in real code.

---

## Chapter 13: Testing & Documentation
**Exercise 13.1 – Unit Test**
- Write `fn add(a, b) -> i32` and a `#[cfg(test)] mod tests` with three `assert_eq!` assertions. Run `cargo test`.

**Exercise 13.2 – Doc Comments**
- Add `///` doc comments to `add`, including an `# Examples` block with a doctest. Run `cargo test` (doctests run too) and `cargo doc --open`.

**Mini-Project 13.3 – Test-Driven FizzBuzz**
- Write tests *first* for a `fn fizzbuzz(n: u32) -> String`, then implement until tests pass.

---

## Chapter 14: Cargo & Publishing
**Exercise 14.1 – New Project**
- `cargo new my_project`, `cd`, `cargo build`, `cargo run`.

**Exercise 14.2 – Add a Dependency**
- Add `rand = "0.8.5"` to `Cargo.toml`, use `rand::random::<u32>()` to generate a number and print it.

**Exercise 14.3 – Crate Metadata (no publish)**
- Fill out `Cargo.toml` `[package]` fields (name, version, authors, edition, description). Don't publish — just verify `cargo build` still works.

---

## Chapter 15: Standard Library & Community
**Exercise 15.1 – HashMap Grades**
- Build the `HashMap` example from the book: insert three students, look up "Bob" with `if let Some(g) = grades.get("Bob")`.

**Exercise 15.2 – Vec & String Utilities**
- Use `std::collections::VecDeque` to simulate a queue: push back 1,2,3, pop front twice, print what's left.

**Exercise 15.3 – Contributing**
- Browse the Rust forums at users.rust-lang.org and read one "help" thread. (Optional: open one issue on a repo you use.)

---

## Chapter 16: Advanced Ownership / Macros / FFI
**Exercise 16.1 – Macro**
- Write `macro_rules! greet { ($name:expr) => { println!("Hello, {}", $name); } }` and call it.

**Exercise 16.2 – FFI (read-only)**
- Copy the `extern "C" { fn printf(...) }` example. Don't modify the unsafe block — just study how the `*const u8` and `\0` termination work.

---

## Chapter 17: Practical Projects
**Mini-Project 17.1 – CLI Greeter**
- Build the `cli-app` from the book: read `env::args()`, if exactly 2 args print "Hello, {name}!", otherwise print usage.

**Mini-Project 17.2 – Web Service (optional)**
- Create a new project, add `actix-web`, and copy the `/hello/{name}` route. Run and hit `http://127.0.0.1:8080/hello/you`.

**Mini-Project 17.3 – Embedded (optional)**
- Skip unless you have hardware; otherwise just read the `#![no_std]` example and note what `#![no_main]` implies.

---

## Chapter 18: Best Practices
**Exercise 18.1 – Idiomatic Refactor**
- Take your FizzBuzz (5.1) and rewrite it using iterators + `match` in a single chain if possible. Compare line counts.

**Exercise 18.2 – Pitfall Fix**
- Reproduce the "unchecked indexing panic" from the book, then fix it with `.get()`. Then reproduce an unnecessary `.clone()` and fix with a reference.

---

## Final Capstone Suggestions

Pick one:

1. **Text-based Todo CLI** — store todos in a `Vec<Todo>` where `Todo` is a struct; add / list / remove via `match` on `env::args()`. Uses: structs, enums (command enum), Result, iterators, match.
2. **Word Frequency Counter** — read a file, split into words, count occurrences in a `HashMap<String, usize>`, print top 10. Uses: HashMap, Result, iterators, borrowing, closures.
3. **Simple Stack VM** — implement a `Vec<i32>`-based stack with `push/pop/add/mul` opcodes. Uses: enums, match, Result, structs, tests.

---

### Suggested pacing
- Chapters 1–4: ~2 hours
- Chapters 5–8: ~3–4 hours
- Chapters 9–11: ~3 hours
- Chapters 12–14: ~2–3 hours
- Chapters 15–18 + Capstone: ~4–6 hours

Would you like me to write out starter templates (skeleton code) for any specific exercise, or a solution key for a subset?
