use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct InputData {
    message: String,
}

#[actix_web::post("/api/send")]
async fn send(data: web::Json<InputData>) -> HttpResponse {
    println!("Received: {}", data.message);
    HttpResponse::Ok().body("Message received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(send)
            .service(Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
