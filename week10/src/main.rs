#![allow(dead_code)]

// Lab 10: The Borrow Checker Game
// Fix each problem one at a time by uncommenting the function call in main()

fn main() {
    println!("Lab 10: Mastering Ownership and Borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // Uncomment problems one at a time:
    // problem_1();
    // problem_2();
    // problem_3();
    // problem_4();
    // problem_5();
    // problem_6();
    // problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
// Error: This function tries to use a value after ownership has moved.
// Fix: Change calculate_length to borrow instead of taking ownership.
//
// Learning goal: Understand move semantics and when to use references
// ============================================================================

fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> (&String, usize) {
    let length = s.len();
    (s, length)
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
// Error: Tries to create a mutable borrow while an immutable borrow exists.
// Fix: Ensure immutable borrows are no longer used before creating mutable borrow.
//
// Learning goal: Understand the "one mutable OR many immutable" rule
// ============================================================================

fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  {}", r1);

    let r2 = &mut s;
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through immutable reference
// ============================================================================
// Error: Tries to mutate a value through an immutable reference.
// Fix: Change both the variable declaration and function signature to accept &mut.
//
// Learning goal: Know when to use &T vs &mut T
// ============================================================================

fn problem_3() {
    println!("Problem 3: Mutating through immutable reference");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
// Error: Creates two mutable references to the same data simultaneously.
// Fix: Use scopes to limit the lifetime of the first mutable borrow.
//
// Learning goal: Control borrow lifetimes with scopes
// ============================================================================

fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  {}", r1);
    }

    let r2 = &mut s;
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
// Error: Returns a reference to data that will be dropped.
// Fix: Return the owned String instead of a reference.
//
// Learning goal: Prevent use-after-free bugs
// ============================================================================

fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello")
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
// Error: Tries to move a value multiple times in a loop.
// Fix: Clone the value in each iteration, or use a reference instead.
//
// Learning goal: Understand ownership with iteration
// ============================================================================

fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &String, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime extension challenge
// ============================================================================
// Error: Reference doesn't live long enough.
// Fix: Restructure so the String lives long enough for the reference.
//
// Learning goal: Understand scope and lifetime relationships
// ============================================================================

fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("inner scope");
    let result = &s;
    println!("  Result: {}", result);
}

// ============================================================================
// TEST SUITE
// ============================================================================
// These tests verify your fixes are correct.
// Run with: cargo test
// ============================================================================

// ============================================================================
// IMPLEMENTATION EXERCISES
// Write these functions from scratch with correct ownership/borrowing
// ============================================================================

fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

fn string_length(s: &str) -> usize {
    s.len()
}

fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_borrows() {
        let s = String::from("testing");
        let (_s_ref, len) = calculate_length(&s);
        assert_eq!(len, 7);
        // s should still be valid here
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_add_to_string_mutates() {
        let mut s = String::from("hello");
        add_to_string(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_create_string_returns_owned() {
        let result = create_string();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_print_with_number_borrows() {
        let data = String::from("Rust");
        // Should work when passed as a reference
        for i in 0..3 {
            print_with_number(&data, i);
        }
        // data should still be valid
        assert_eq!(data, "Rust");
    }
}
