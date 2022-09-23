#![feature(proc_macro_hygiene)]
#![allow(unused_braces)]
use actix_files::Files;
use actix_web::{
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
mod server;
mod plot;
use server::create_server;


async fn server1(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    actix_web_actors::ws::start(create_server(), &req, stream)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/lib", "./static").show_files_listing())
            .service(web::resource("/websocket/").route(web::get().to(server1)))
            .service(Files::new("/", "./dist").index_file("index.html"))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
