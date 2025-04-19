use std::{env, process::Command};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    input_file_path: String,
    output_file_path: String,
}

#[post("/process-video")]
async fn process_video(info: web::Json<Info>) -> impl Responder {
    let input_file_path = &info.input_file_path;
    let output_file_path = &info.output_file_path;

    if input_file_path.is_empty() || output_file_path.is_empty() {
        if input_file_path.is_empty() { return HttpResponse::BadRequest().body("Bad Request: Missing Input File Path."); }
        return HttpResponse::BadRequest().body("Bad Request: Missing Output File Path.");
    }

    let result = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file_path)
        .arg("-vf")
        .arg("scale=iw*sar:360:force_original_aspect_ratio=decrease")
        .arg(output_file_path)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                HttpResponse::Ok().body("Processing Finished Succesfully.")
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                eprintln!("FFmpeg error: {}", error_message);
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", error_message))
            }
        }
        Err(e) => {
            eprintln!("Error executing FFmpeg: {}", e);
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", e))
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
    let default_port: u16 = 3000;
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(default_port);
    let binding_address = ("0.0.0.0", port);

    match HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
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