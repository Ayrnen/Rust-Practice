use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use serde::ser;

#[actix_web::main]
async fn main() {
    let server_closure = ||{
        App::new().route("/", web::get().to(get_index)) 
    };

    let server = HttpServer::new(server_closure);
    println!("Server is listening on http://localhost:3000...");

    server.bind("127.0.0.1:3000").expect("Error binding server to")
    .run().await.expect("error running the server")
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("It works!")
}