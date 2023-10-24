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
    match general_purpose::STANDARD.decode(image_base_64) {
        Ok(image_bytes) => match leptess::LepTess::new(None, "eng") {
            Ok(mut lt) => match lt.set_image_from_mem(&image_bytes) {
                Ok(_) => match lt.get_utf8_text() {
                    Ok(ocrText) => Ok(ocrText),
                    Err(utf8Error) => Err(format!("{utf8Error}")),
                },
                Err(pixError) => Err(format!("{pixError}")),
            },
            Err(tesseractError) => Err(format!("{tesseractError}")),
        },
        Err(decodeError) => Err(format!("{decodeError}")),
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
        Err(errMsg) => HttpResponse::InternalServerError().json(ResponseType {
            result: format!("{errMsg}"),
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
