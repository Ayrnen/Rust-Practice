use tokio;
use std::{time};

async fn print_odd(){
    let total = 10;
    let mut count = 0;
    let mut odd = 1;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("print odd value:  {}", odd);
        odd += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

async fn print_even(){
    let total = 10;
    let mut count = 0;
    let mut even = 2;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("print even value:  {}", even);
        even += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

#[tokio::main]
async fn main() {
    // odd and even are objects of type "Future"
    let odd = print_odd();

    let even = print_even();

    println!("begin counting...");
    // This block will wait until both functions finish running 
    tokio::join!(odd, even);
    println!("End of Script");
}
