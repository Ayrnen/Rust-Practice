fn gcd(mut n:i32, mut m:i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        //exchange the value of m and n if m > n
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        
        m %= n;

    }
    
    n // never put semicolon when returning a value 
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}

use std::str::FromStr;

fn main() {
    // declare new vector (dynamic array)
    let mut numbers = Vec::new();
    // grab args from cmd - skipping the first one (file loc)
    let args = std::env::args().skip(1);
    for arg in args {
        //convert args from strings to numbers
        let res = i32::from_str(&arg);
        match res {
            Ok(num) => {
                numbers.push(num);
            }
            Err(e) => {
                eprintln!("Arg/Parsing error: {}", e);
                std::process::exit(1);
            }
        }
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd num1 num2 ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("the greatest common devisor of {:?} is {}", numbers, d)
}