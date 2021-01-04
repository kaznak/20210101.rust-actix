use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// This struct represents state
struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}

async fn index_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

async fn app_index() -> impl Responder {
    "Hello world!"
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        app_name: String::from("Actix-web"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/app").route("/index.html", web::get().to(app_index)))
            .app_data(counter.clone())
            .service(index)
            .route("/counter", web::get().to(index_counter))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
