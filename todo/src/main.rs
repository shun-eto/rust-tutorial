use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello World";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:9000")?
        .run()
        .await
}
