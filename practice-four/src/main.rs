use actix_web::{dev::Server, web::{self, post}, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters{
    n: i32,
    m: i32,
}

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

#[actix_web::main]
async fn main() {
    let server_closure = ||{
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    };

    let server = HttpServer::new(server_closure);
    println!("Server is listening on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000")
        .expect("Error binding server to")
        .run()
        .await
        .expect("error running the server")
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing GCD with 0 is impossible.");
    }
    
    let response = format!("The GCD of {} and {} is {}", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}