# Chapter 3: Understanding Ownership & Borrowing
## Exercise 3.1 – Ownership Detective

Write three small snippets: one that moves a String, one that copies an i32, one that borrows a String.

For each, add a comment explaining why the code compiles or fails. Then deliberately break one (use a moved value) and read the compiler error carefully.

## Exercise 3.2 – Length Calculator

Write fn calculate_length(s: &String) -> usize and call it without transferring ownership. Print the original string afterward to prove it still lives.

## Exercise 3.3 – Mutable Borrow

Write fn add_exclamation(s: &mut String) and call it. Then try calling it twice on the same variable in one statement (e.g., add_exclamation(&mut s); add_exclamation(&mut s);) and observe that it's fine — then try holding two &mut at once and see the error.

## Mini-Project 3.4 – String Stats

Write a function fn stats(s: &str) -> (usize, usize) returning (character count, word count). Do not take ownership. Call it on a String and on a string literal.
