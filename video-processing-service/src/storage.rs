use std::{
    process::Command,
    fs,
    sync::Arc,
    path,
};

use object_store::{
    gcp::GoogleCloudStorageBuilder,
    path::Path,
    ObjectStore,
};

const RAW_BUCKET         : &str = "yt-raw-videos-deepencoding-clone";
const PROCESSED_BUCKET   : &str = "yt-processed-videos-deepencoding-clone";
pub const LOCAL_RAW_DIR      : &str = "./raw-videos";
pub const LOCAL_PROCESSED_DIR: &str = "./processed-videos";

#[derive(Clone)]
pub struct GcsClient {
    raw_store: Arc<dyn ObjectStore>,
    processed_store: Arc<dyn ObjectStore>,
}

impl GcsClient {
    pub fn new() -> Self {
        let raw_store = GoogleCloudStorageBuilder::new()
            .with_bucket_name(RAW_BUCKET)
            .build()
            .expect("Failed to create raw bucket client");
        
        let processed_store = GoogleCloudStorageBuilder::new()
            .with_bucket_name(PROCESSED_BUCKET)
            .build()
            .expect("Failed to create processed bucket client");

        GcsClient::setup_directories().expect("Failed to create directories.");

        Self {
            raw_store: Arc::new(raw_store),
            processed_store: Arc::new(processed_store)
        }
    }

    fn setup_directories() -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(LOCAL_RAW_DIR).expect("Failed to create raw local directory.");
        fs::create_dir_all(LOCAL_PROCESSED_DIR).expect("Failed to create processed local directory.");
        Ok(())
    }

    pub async fn download_raw_video(&self, raw_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let object_path = Path::from(raw_file_name);
        let data = self.raw_store.get(&object_path).await.expect("Failed to download raw video from GCS.");
        let bytes = data.bytes().await.expect("Failed to collect raw video data into bytes.");

        let local_path = format!("{}/{}", LOCAL_RAW_DIR, raw_file_name);
        match fs::write(&local_path, bytes) {
            Ok(()) => {
                println!("gs://{}/{} downloaded to {}/{}", RAW_BUCKET, raw_file_name, LOCAL_RAW_DIR, raw_file_name);
                Ok(())
            },
            Err(e) => {
                eprintln!("Failed to save raw video: {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn convert_video(raw_video_name: &str, processed_video_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let input_path = format!("{}/{}", LOCAL_RAW_DIR, raw_video_name);
        let output_path = format!("{}/{}", LOCAL_PROCESSED_DIR, processed_video_name);
    
        let result = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_path)
            .arg("-vf")
            .arg("scale=iw*sar:360:force_original_aspect_ratio=decrease")
            .arg(&output_path)
            .output();

        match result {
            Ok(output) => {
                if !output.status.success() {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    eprintln!("FFmpeg error: {}", error_message);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error executing FFmpeg: {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn upload_processed_video(&self, processed_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let local_path = format!("{}/{}", LOCAL_PROCESSED_DIR, processed_file_name);
        let content = match fs::read(&local_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to read processed file path. ");
                return Err(Box::new(e));
            }
        };

        let object_path = Path::from(processed_file_name);
        match self.processed_store.put(&object_path, content.into()).await {
            Ok(_) => {
                println!("{}/{} uploaded to gs://{}/{}", LOCAL_PROCESSED_DIR, processed_file_name, PROCESSED_BUCKET, processed_file_name);
                Ok(())
            },
            Err(e) => {
                eprintln!("Failed to upload video to bucket. ");
                Err(Box::new(e))
            }
        }
    }

    pub fn delete_local_file(local_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if path::Path::new(local_file_path).exists() {
            match fs::remove_file(local_file_path) {
                Ok(_) => {
                    println!("File deleted at {}", local_file_path);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to delete file at {}: {}", local_file_path, e);
                    Err(Box::new(e))
                }
            }
        } else {
            println!("File not found at {}, skipping the delete.", local_file_path);
            Ok(())
        }
    }
}