fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

use std::str::FromStr;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for arg in std::env::args().skip(1) {
        match i32::from_str(&arg) {
            Ok(num) => numbers.push(num),
            Err(e) => {
                eprintln!("Arg/Parsing error: {}", e);
                std::process::exit(1);
            }
        }
    }

    if numbers.is_empty() {
        eprintln!("Usage: gcd num1 num2 ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}