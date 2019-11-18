use actix_web::{App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Session {
    id: String,
    username: String,
}

#[get("/session")]
fn session_mock() -> impl Responder {
    HttpResponse::Ok().json(Session {
        id: String::from("1"),
        username: String::from("denistakeda"),
    })
}

fn main() {
    HttpServer::new(|| {
      App::new()
        .service(session_mock)
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .run()
    .unwrap();
}
