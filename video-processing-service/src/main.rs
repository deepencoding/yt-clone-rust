use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose, Engine};
use serde::Deserialize;

mod storage;
use storage::GcsClient;

mod firestore;
use firestore::{
    DbService,
    Video,
    VideoStatus,
};

#[derive(Deserialize)]
struct MessageData {
    data: String,
}

#[derive(Deserialize)]
struct PubSubMessage {
    message: MessageData,
}

#[post("/process-video")]
async fn process_video(pubsub: web::Json<PubSubMessage>, storage_client: web::Data<GcsClient>, firestore_client: web::Data<DbService>) -> impl Responder {
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

    let video_id = match raw_file_name.split('.').next() {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Invalid filename format."),
    };
    
    // Extract UID (first part of videoId before the dash)
    let uid = match video_id.split('-').next() {
        Some(uid) => uid,
        None => return HttpResponse::BadRequest().body("Invalid video ID format."),
    };

    match firestore_client.is_video_new(video_id).await {
        false => { return HttpResponse::BadRequest().body("Bad Request: video already processing or processed."); },
        true => {
            // Create video document with status "processing"
            let video = Video {
                id: video_id.to_owned(),
                uid: uid.to_owned(),
                filename: raw_file_name.to_owned(),
                status: VideoStatus::Processing,
                title: "".to_owned(),
                description: "".to_owned(),
            };
            firestore_client.set_video(video).await.expect("Failed to update/create video doc."); 
        },
    };
    
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
        Ok(()) => {
            let video = Video {
                id: video_id.to_owned(),
                uid: uid.to_owned(),
                filename: processed_file_name.to_owned(),
                status: VideoStatus::Processed,
                title: "".to_owned(),
                description: "".to_owned(),
            };
            firestore_client.set_video(video).await.expect("Failed to update video doc to processed.");
            HttpResponse::Ok().body("Processing Complete.")
        },
        Err(e) => {
            eprintln!("Processed video Upload error: {}", e);
            GcsClient::delete_local_file(raw_file_name).expect("Failed to delete raw video.");
            GcsClient::delete_local_file(&processed_file_name).expect("Failed to delete processed video.");
            HttpResponse::InternalServerError().body("Failed to upload processed video.")
        }
    }
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
    let firestore_client = DbService::new().await;

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
            .app_data(web::Data::new(firestore_client.clone()))
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