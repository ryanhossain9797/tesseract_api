use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Deserialize)]
struct RequestType {
    image_base_64: String,
}

#[derive(Debug, Serialize)]
struct ResponseType {
    result: String,
}

async fn process(request: web::Json<RequestType>) -> impl Responder {
    let image_bytes = general_purpose::STANDARD.decode(request.image_base_64.clone()).unwrap();

    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image_from_mem(&image_bytes).unwrap();
    let text = lt.get_utf8_text().unwrap();

    let response_type = ResponseType {
        result: format!("{text}"),
    };

    HttpResponse::Ok().json(response_type)
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
            .route("/health", web::get().to(health))
            .route("/process", web::post().to(process)))
        .bind("0.0.0.0:9876")?
        .run()
        .await
}
