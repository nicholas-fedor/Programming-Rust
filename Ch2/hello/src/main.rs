fn main() {
    println!("Hello, world!");
}

// Pg 8 - Addition of Euclid's algorithm to compute greatest common divisor of two integers.

// Function gcd takes two parameters n and m (u64 = unsigned 64-bit integers) and returns a u64 value.
// Placing the "mut" keyword in front of the variables allows them to be mutable.
// *By default in Rust, once a variable is initialized, it cannot be changed.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // assert! is a macro function that verifies that neither argument is zero.
    // it checks that its argument is true, and if not, terminates the program and outputs a debug message.
    assert!(n != 0 && m != 0);
    // while loop containing an if statement and an assignment.
    while m != 0 {
        if m < n {
            // let statement declares a local variable.
            let t = m;
            m = m;
            n = t;
        }
        m = m % n;
    }
    // If a function body ends with an expression that is not followed by a semicolon, that's the function's return value.
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 *  13 * 19), 3 * 11);
}