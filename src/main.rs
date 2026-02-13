// AI Agent Demo - Functions Generated with AI Assistance
// This demonstrates how AI tools like GitHub Copilot can help generate Rust code

use std::collections::HashMap;

fn main() {
    println!("=== AI Agent Demo - Rust Code Generation ===\n");
    
    // Demo calculator functions
    println!("1. Calculator Functions:");
    println!("   Add: 10 + 5 = {}", add(10, 5));
    println!("   Multiply: 10 * 5 = {}", multiply(10, 5));
    println!("   Factorial: 5! = {}", factorial(5));
    
    // Demo string utilities
    println!("\n2. String Utilities:");
    let text = "Hello, Rust!";
    println!("   Original: '{}'", text);
    println!("   Reversed: '{}'", reverse_string(text));
    println!("   Word count: {}", count_words(text));
    println!("   Is palindrome 'racecar': {}", is_palindrome("racecar"));
    
    // Demo data structure operations
    println!("\n3. Data Structure Operations:");
    let numbers = vec![5, 2, 8, 1, 9, 3];
    println!("   Original vector: {:?}", numbers);
    println!("   Sum: {}", sum_vector(&numbers));
    println!("   Average: {:.2}", average_vector(&numbers));
    println!("   Max value: {:?}", find_max(&numbers));
    
    // Demo hashmap operations
    println!("\n4. HashMap Operations:");
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    println!("   Scores: {:?}", scores);
    println!("   Highest score: {:?}", find_highest_score(&scores));
}

// Calculator functions

/// Adds two numbers together
/// 
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers
/// 
/// # Examples
/// ```
/// assert_eq!(multiply(3, 4), 12);
/// ```
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Calculates the factorial of a number
/// 
/// # Examples
/// ```
/// assert_eq!(factorial(5), 120);
/// ```
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// String utility functions

/// Reverses a string
/// 
/// # Examples
/// ```
/// assert_eq!(reverse_string("hello"), "olleh");
/// ```
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Counts the number of words in a string
/// 
/// # Examples
/// ```
/// assert_eq!(count_words("Hello world"), 2);
/// ```
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Checks if a string is a palindrome
/// 
/// # Examples
/// ```
/// assert_eq!(is_palindrome("racecar"), true);
/// assert_eq!(is_palindrome("hello"), false);
/// ```
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

// Vector operations

/// Calculates the sum of all numbers in a vector
/// 
/// # Examples
/// ```
/// assert_eq!(sum_vector(&vec![1, 2, 3, 4]), 10);
/// ```
fn sum_vector(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

/// Calculates the average of numbers in a vector
/// 
/// # Examples
/// ```
/// assert_eq!(average_vector(&vec![2, 4, 6, 8]), 5.0);
/// ```
fn average_vector(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    sum_vector(numbers) as f64 / numbers.len() as f64
}

/// Finds the maximum value in a vector
/// 
/// # Examples
/// ```
/// assert_eq!(find_max(&vec![1, 5, 3, 2]), Some(&5));
/// ```
fn find_max(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().max()
}

// HashMap operations

/// Finds the entry with the highest score in a HashMap
/// 
/// # Examples
/// ```
/// let mut scores = HashMap::new();
/// scores.insert("Alice", 95);
/// scores.insert("Bob", 87);
/// assert_eq!(find_highest_score(&scores), Some(("Alice", 95)));
/// ```
fn find_highest_score<'a>(scores: &'a HashMap<&'a str, i32>) -> Option<(&'a str, i32)> {
    scores.iter().max_by_key(|(_, &score)| score).map(|(&name, &score)| (name, score))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello world"), 2);
        assert_eq!(count_words("one"), 1);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("  spaces  between  "), 2);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_sum_vector() {
        assert_eq!(sum_vector(&vec![1, 2, 3, 4]), 10);
        assert_eq!(sum_vector(&vec![]), 0);
        assert_eq!(sum_vector(&vec![-1, 1]), 0);
    }

    #[test]
    fn test_average_vector() {
        assert_eq!(average_vector(&vec![2, 4, 6, 8]), 5.0);
        assert_eq!(average_vector(&vec![]), 0.0);
        assert_eq!(average_vector(&vec![5]), 5.0);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&vec![1, 5, 3, 2]), Some(&5));
        assert_eq!(find_max(&vec![]), None);
        assert_eq!(find_max(&vec![-5, -1, -10]), Some(&-1));
    }

    #[test]
    fn test_find_highest_score() {
        let mut scores = HashMap::new();
        scores.insert("Alice", 95);
        scores.insert("Bob", 87);
        scores.insert("Charlie", 92);
        
        let result = find_highest_score(&scores);
        assert!(result.is_some());
        let (name, score) = result.unwrap();
        assert_eq!(name, "Alice");
        assert_eq!(score, 95);
    }
}
