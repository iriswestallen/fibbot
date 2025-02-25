use std::env;

fn main() {
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);
}