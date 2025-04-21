use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose, Engine};
use serde::Deserialize;

mod storage;
use storage::GcsClient;


#[derive(Deserialize)]
struct MessageData {
    data: String,

    // #[serde(rename="messageId")]
    // message_id: String,
}

#[derive(Deserialize)]
struct PubSubMessage {
    message: MessageData,
}

// #[derive(Deserialize)]
// struct GcsNotification {
//     name: String,
//     bucket: String,
// }

#[post("/process-video")]
async fn process_video(pubsub: web::Json<PubSubMessage>, storage_client: web::Data<GcsClient>) -> impl Responder {
    let decoded = match general_purpose::STANDARD.decode(&pubsub.message.data) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Base64 decode error: {}", err);
            return HttpResponse::BadRequest().body("Bad Request: Invalid base64.");
        }
    };

    let msg_str = match String::from_utf8(decoded) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("UTF-8 decode error: {}", err);
            return HttpResponse::BadRequest().body("Bad Request: Invalid UTF-8.");
        }
    };

    let payload: serde_json::Value = match serde_json::from_str(&msg_str) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("JSON parse error: {}", err);
            return HttpResponse::BadRequest().body("Bad Request: Invalid JSON.");
        }
    };

    let raw_file_name = match payload.get("name").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return HttpResponse::BadRequest().body("Bad Request: Missing filename."),
    };
    let processed_file_name = format!("processed-{}", raw_file_name);

    match storage_client.download_raw_video(raw_file_name).await {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Raw video download error: {}", e);
            return HttpResponse::InternalServerError().body("Failed to download raw video.");
        },
    };

    match GcsClient::convert_video(raw_file_name, &processed_file_name) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Video conversion error: {}", e);
            GcsClient::delete_local_file(raw_file_name).expect("Failed to delete raw video.");
            GcsClient::delete_local_file(&processed_file_name).expect("Failed to delete processed video.");
            return HttpResponse::InternalServerError().body("Failed to convert video.");
        }
    }

    match storage_client.upload_processed_video(&processed_file_name).await {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Processed video Upload error: {}", e);
            GcsClient::delete_local_file(raw_file_name).expect("Failed to delete raw video.");
            GcsClient::delete_local_file(&processed_file_name).expect("Failed to delete processed video.");
            return HttpResponse::InternalServerError().body("Failed to upload processed video.");
        }
    };
    
    HttpResponse::Ok().body("Processing Complete.")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
    let storage_client = GcsClient::new();

    let default_port: u16 = 3000;
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(default_port);
    let binding_address = ("0.0.0.0", port);

    match HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .app_data(web::Data::new(storage_client.clone()))
            .service(process_video)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(binding_address) {
        Ok(server) => {
            println!("Video Processing Service listening at http://{}:{}", binding_address.0, binding_address.1);
            server.run().await
        }
        Err(e) => {
            eprintln!("Error starting server: {}", e);
            Err(e)
        }
    }
}