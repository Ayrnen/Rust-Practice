use std::thread;

fn run_thread(){
    let s = "string in the run_thread";
    let mut v  = vec![1, 2, 3];
    let handle_closure = move || {
        thread::sleep(std::time::Duration::from_millis(2000));
        println!("The content of s is :{}", s);
        v.push(4);
        println!("the content of v is {:?}", v);
        println!("error here: {}", v[6])
    };
    let handle = thread::spawn(handle_closure);

    handle.join().unwrap();
}

fn main() {
    run_thread();
}
