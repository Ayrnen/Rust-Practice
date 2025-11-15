fn func_with_closure<G>(f: G) where G:FnOnce(&str), {
    f("hello world!");
}

fn main() {
    let s: &str = "The content of x is: ";
    let print_s_closure = |x:&str|{
        println!("{} {}", s, x);    
    };

    func_with_closure(print_s_closure);

}