use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct RequestType {
    image_base_64: String,
}

#[derive(Debug, Serialize)]
struct ResponseType {
    result: String,
}

async fn get_text_from_image(image_base_64: String) -> Result<String, String> {
    let image_bytes = general_purpose::STANDARD
        .decode(image_base_64)
        .map_err(|err| err.to_string())?;

    let mut lt = leptess::LepTess::new(None, "eng").map_err(|err| err.to_string())?;

    lt.set_image_from_mem(&image_bytes)
        .map_err(|err| err.to_string())?;

    match lt.get_utf8_text() {
        Ok(ocr_text) => Ok(ocr_text),
        Err(utf_8_error) => Err(format!("{utf_8_error}")),
    }
}

async fn process(request: web::Json<RequestType>) -> impl Responder {
    let text_result = get_text_from_image(request.image_base_64.clone()).await;

    match text_result {
        Ok(text) => {
            let response_type = ResponseType {
                result: format!("{text}"),
            };

            HttpResponse::Ok().json(response_type)
        }
        Err(err_msg) => HttpResponse::InternalServerError().json(ResponseType {
            result: format!("{err_msg}"),
        }),
    }
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/process", web::post().to(process))
    })
    .bind("0.0.0.0:9876")?
    .run()
    .await
}
