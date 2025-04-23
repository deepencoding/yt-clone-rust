use firestore::*;
use serde::{
    Deserialize,
    Serialize,
};

const VIDEO_COLLECTION_NAME: &'static str = "videos";

#[derive(Default, PartialEq, Copy, Debug, Clone, Deserialize, Serialize)]
pub enum VideoStatus {
    Processing,
    Processed,
    #[default]
    Undefined,
}

// Video document structure
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Video {
    pub id: String,
    pub uid: String,
    pub filename: String,
    pub status: VideoStatus,
    pub title: String,
    pub description: String,
}

#[derive(Clone)]
pub struct DbService {
    client: FirestoreDb,
}

impl DbService {
    pub async fn new() -> Self {
        Self {
            client: FirestoreDb::with_options_service_account_key_file(
                FirestoreDbOptions::new("yt-clone-rust".to_owned()),
                "./firestore-service-acc.json".parse().unwrap()
            ).await.unwrap()
        }
    }

    async fn get_video(&self, video_id: &str) -> FirestoreResult<Video> {
        Ok(
            self.client.fluent()
                .select()
                .by_id_in(VIDEO_COLLECTION_NAME)
                .obj()
                .one(&video_id)
                .await.expect("Failed to fecth video document.").unwrap_or_default()
        )
    }

    pub async fn set_video(&self, video: Video) -> FirestoreResult<()> {
        self.client.fluent()
            .update()
            .fields(paths!(Video::{ id, uid, filename, status, title, description }))
            .in_col(VIDEO_COLLECTION_NAME)
            .document_id(video.id.clone())
            .object(&video)
            .execute::<Video>().await.expect("Failed to set the video.");
        Ok(())
    }

    pub async fn _get_all(&self) -> FirestoreResult<Vec<Video>> {
        self.client.fluent()
            .select()
            .from(VIDEO_COLLECTION_NAME)
            .obj()
            .query()
            .await
    }

    pub async fn _delete_video_by_id(&self, video_id: &str) -> FirestoreResult<()> {
        self.client.fluent()
            .delete()
            .from(VIDEO_COLLECTION_NAME)
            .document_id(video_id)
            .execute()
            .await
    }

    pub async fn is_video_new(&self, video_id: &str) -> bool {
        match self.get_video(video_id).await {
            Ok(video) => { video.status == VideoStatus::Undefined },
            Err(_) => { false }
        }
    }
}