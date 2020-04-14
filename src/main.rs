extern crate juniper;

use std::io;
use actix_web::{web, App, Responder, HttpResponse, HttpServer};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

fn main() -> io::Result<()> {
    HttpServer::new( || {
        App::new()
            .route("/", web::get().to(index))
    } )
    .bind("localhost:8080")?
    .run()
}
