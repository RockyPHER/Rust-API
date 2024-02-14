use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod waifu;

#[derive(Debug)]
struct ApiResponse {
    message: String,
}

async fn get_waifus() -> impl Responder {
    let waifus = vec![
        waifu::Waifu {
            name: "Rem".to_string(),
            age: 16,
            image_url: "https://i.pinimg.com/564x/7d/52/09/7d5209a6d70162fd48e2460042632190.jpg"
                .to_string(),
        },
        waifu::Waifu {
            name: "Erina".to_string(),
            age: 18,
            image_url: "https://i.pinimg.com/564x/b4/30/39/b43039a32ec9915c29cf6386f730954c.jpg"
                .to_string(),
        },
    ];

    HttpResponse::Ok().json(waifus)
}

async fn add_waifu(new_waifu: web::Json<waifu::Waifu>) -> impl Responder {
    let mut waifus: Vec<waifu::Waifu> = Vec::new();

    if waifus.inter().any(|w| w.name == new_waifu.name) {
        return HttpResponse::Conflict().json(ApiResponse {
            message: "Waifu with the same name already exists".to_string(),
        });
    }

    waifus.push(new_waifu.into_inner());

    HttpResponse::Created().json(ApiResponse {
        message: "Waifu added successfully".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/waifus", web::get().to(get_waifus))
            .route("/waifus", web::post().to(add_waifu))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
