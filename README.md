# 🎬 YouTube Clone (Rust + Next.js)

**Stack**: Rust (Actix Web) • TypeScript • Next.js • Firestore • GCP (Cloud Storage, Pub/Sub, Cloud Run) • Docker
[🚀 Live Demo](https://yt-web-client-543501541359.asia-south2.run.app/)

---

## 🚀 Project Overview

A full-stack YouTube-style application featuring a Rust-based backend (Actix Web) and a Next.js/TypeScript frontend. Built on Google Cloud infrastructure for scalable, event-driven video upload, transcoding, storage, and playback. Enables seamless user experience with real-time metadata, high-concurrency support, and automated scaling via Cloud Run.

---

## 🎯 Key Features

- **Rust + Actix Web Backend**  
  - Type-safe, high-performance HTTP API serving video metadata, upload endpoints, authentication, and playback session management.
  - Clean separation of concerns: handlers, services, data access layers, leveraging Rust’s ownership model for reliability.

- **Next.js Frontend (TypeScript)**  
  - Server-side rendering (SSR) and client-side interactions via React and tRPC (or REST) for video browsing, watch pages, and upload UI.
  - Responsive UI for browsing video lists, watch experience with adaptive player, and upload progress indicators.

- **Event-Driven Video Pipeline**  
  - Upload endpoint stores raw uploads in Google Cloud Storage.
  - Publishes a message to Pub/Sub topic `video-uploaded`.
  - A Dockerized FFmpeg transcoding service (written in Rust or another language) subscribes to Pub/Sub, pulls raw video, transcodes into HLS (or other formats), generates thumbnails, and writes outputs back to Cloud Storage.
  - After transcoding, updates Firestore metadata via backend API (or a Cloud Function) to mark video ready for playback.

- **Google Cloud Firestore & Storage Integration**  
  - Firestore for video metadata (titles, descriptions, user data, watch counts, comments references).
  - Cloud Storage for raw uploads and processed media (HLS segments, thumbnail images).
  - Uses an `object_store` abstraction layer (Rust crate or custom module) to interact with GCP Storage buckets, improving code modularity.

- **Automated Scaling & Deployment**  
  - Backend and transcoder services containerized with Docker.
  - Deployed to Google Cloud Run with concurrency settings and autoscaling.
  - Frontend deployed on Cloud Run or a static hosting service (e.g., Vercel) with environment configured to point to the backend API.
  - Pub/Sub ensures decoupling between upload and processing, enabling robust high-concurrency handling.

- **Security & Reliability**  
  - Authentication (e.g., JWT) for upload and metadata APIs.
  - Firestore security rules to restrict writes/reads appropriately.
  - Retry policies on Pub/Sub subscriptions for transient failures.
  - Logging and monitoring integration (Cloud Logging) to track pipeline health.

---

## 🏗️ Architecture Diagram

```
[User Browser]
↓ (Next.js SSR / API calls via tRPC or REST)
[Next.js Frontend on Cloud Run/Vercel]
↔ (HTTPS)
[Rust Actix Web Backend on Cloud Run]
↔ Firestore (metadata)
↔ Cloud Storage (raw uploads, thumbnails, HLS segments)

Upload Flow:
[Frontend Upload UI] → [Backend Upload Handler] → Cloud Storage (raw) → Pub/Sub “video-uploaded”

Transcode Flow:
Pub/Sub → [Transcoder Service on Cloud Run / Subscriber] → FFmpeg → Cloud Storage (processed) → Firestore update via backend API

Playback Flow:
[Frontend fetches metadata from Backend] → Cloud Storage (signed URLs or public URLs) → Video player streams HLS segments
```

---

## 🛠️ Tech Stack

- **Backend**  
  - Rust, Actix Web framework  
  - `tokio` runtime for async I/O  
  - Firestore client (e.g., via Google Cloud Rust SDK or custom wrapper)  
  - `object_store` crate or custom module to abstract Cloud Storage operations  
  - Pub/Sub client to publish/subscribe events (Rust SDK or HTTP-based)  
  - JWT authentication (e.g., `jsonwebtoken` crate)  
  - Docker for containerization

- **Video Processing Service**  
  - Dockerized FFmpeg commands orchestrated by a small Rust (or other) subscriber service  
  - Pub/Sub subscription logic to trigger transcoding  
  - Cloud Storage for reading raw blobs and writing processed outputs

- **Frontend**  
  - Next.js (app/router or pages) in TypeScript  
  - React components for video list, watch page, upload form, user profile  
  - tRPC or REST client to call Rust backend endpoints  
  - CSS framework or utility-first CSS (Tailwind, UnoCSS) for styling  
  - Vercel or Cloud Run hosting

- **Infrastructure & Deployment**  
  - Google Cloud Platform: Firestore, Cloud Storage, Pub/Sub, Cloud Run  
  - Dockerfiles for backend & transcoder  
  - CI/CD pipelines (GitHub Actions) to build images, run tests, and deploy to Cloud Run  
  - Environment management: secrets (service account keys or Workload Identity), configuration via environment variables

---

## 🧪 Getting Started (Local Development)

### Prerequisites

- Rust toolchain (stable) installed (`rustup`, `cargo`)  
- Node.js & npm / pnpm / yarn for frontend  
- Docker installed (for local container tests)  
- Google Cloud SDK (`gcloud`) configured with a project (optional for local emulation)  
- Emulator tools for Firestore / Pub/Sub (if desired) or real GCP resources

### Clone the Repo

```bash
git clone https://github.com/deepencoding/yt-clone-rust.git
cd yt-clone-rust
````

### 1. Backend Setup

1. **Environment Variables**
   Create a `.env` (or `.env.local` for local) in backend folder with:

   ```text
   GCP_PROJECT_ID=your-gcp-project-id
   FIRESTORE_CREDENTIALS_JSON=/path/to/service-account.json  # or set GOOGLE_APPLICATION_CREDENTIALS
   STORAGE_BUCKET_RAW=your-raw-videos-bucket
   STORAGE_BUCKET_PROCESSED=your-processed-videos-bucket
   PUBSUB_TOPIC_VIDEO_UPLOADED=video-uploaded
   PUBSUB_SUBSCRIPTION_TRANSCODER=video-transcode-sub
   JWT_SECRET=your_jwt_secret_key
   BACKEND_PORT=8080
   ```

   * If using local emulator, configure emulator host endpoints accordingly.

2. **Build & Run Locally**

   ```bash
   cd backend
   cargo build --release
   # Or for development:
   cargo run
   ```

   The server listens on `http://localhost:8080` by default.

3. **Emulator (Optional)**

   * Firestore emulator: start via `gcloud beta emulators firestore start` or use Docker-based emulator.
   * Pub/Sub emulator: use `gcloud beta emulators pubsub start`.
   * Configure environment variables like `FIRESTORE_EMULATOR_HOST`, `PUBSUB_EMULATOR_HOST`.
   * Cloud Storage: consider using a local emulation or real buckets.

4. **Database Initialization**

   * No relational database needed; ensure Firestore collections exist implicitly when written.
   * Ensure Firestore security rules set to allow local tests or adjust for production.

### 2. Frontend Setup

1. **Navigate to Frontend**

   ```bash
   cd frontend
   ```

2. **Install Dependencies**

   ```bash
   npm install
   # or pnpm install / yarn
   ```

3. **Environment Variables**
   Create `.env.local` with:

   ```text
   NEXT_PUBLIC_BACKEND_URL=http://localhost:8080
   NEXT_PUBLIC_FIRESTORE_PROJECT_ID=your-gcp-project-id  # if frontend interacts directly with Firestore for public reads
   ```

   * For production, set `NEXT_PUBLIC_BACKEND_URL` to deployed backend endpoint.

4. **Run Dev Server**

   ```bash
   npm run dev
   ```

   Open [http://localhost:3000](http://localhost:3000) to view the app.

### 3. Local Video Processing (Optional)

* To test transcoding locally, you can run a local subscriber:

  1. Build the transcoder service (if in Rust):

     ```bash
     cd transcoder
     cargo build --release
     ```
  2. Configure environment variables similar to backend, pointing to emulator or real buckets.
  3. Run:

     ```bash
     cargo run
     ```
  4. Upload a sample video via backend API; confirm a Pub/Sub message is published and transcoder picks it up, producing processed outputs in storage emulator or bucket.

### 4. Docker & Containerized Local Testing

1. **Build Docker Images**

   ```bash
   # From repo root
   docker build -t yt-backend:local -f backend/Dockerfile .
   docker build -t yt-transcoder:local -f transcoder/Dockerfile .
   ```
2. **Run Containers Locally**

   * For backend:

     ```bash
     docker run -e GCP_PROJECT_ID=... \
                -e GOOGLE_APPLICATION_CREDENTIALS=/secrets/service-account.json \
                -e other env... \
                -p 8080:8080 \
                -v /local/path/to/service-account.json:/secrets/service-account.json \
                yt-backend:local
     ```
   * For transcoder:

     ```bash
     docker run -e ... yt-transcoder:local
     ```
   * Ensure emulator endpoints or real GCP connectivity is configured.

---

## 🧩 Deployment

### 1. Google Cloud Build & Cloud Run

* **Backend**

  * Create a Cloud Build config or GitHub Actions workflow to build a Docker image and push to Container Registry / Artifact Registry.
  * Deploy to Cloud Run:

    ```bash
    gcloud run deploy yt-clone-backend \
      --image gcr.io/your-gcp-project/yt-backend:latest \
      --region asia-south2 \
      --set-env-vars GCP_PROJECT_ID=...,STORAGE_BUCKET_RAW=...,JWT_SECRET=...,…
    ```
  * Configure concurrency, memory, and autoscaling parameters as needed.

* **Transcoder Service**

  * Similarly, build and deploy to Cloud Run with a subscription to Pub/Sub (use Cloud Run Pub/Sub push subscription or a pull subscriber within the service).
  * Ensure the service has permissions to read/write Storage and update Firestore (via service account IAM roles).

* **Frontend**

  * Deploy Next.js app to Vercel or Cloud Run (static export or serverless).
  * Set environment variables: `NEXT_PUBLIC_BACKEND_URL=https://yt-clone-backend-...run.app`.

### 2. Firestore & Storage Setup

* **Firestore**

  * In GCP console, enable Firestore in native mode.
  * Define security rules: e.g., allow read for public metadata, restrict writes to authenticated users or backend service account.

* **Cloud Storage**

  * Create two buckets: `yt-clone-raw-uploads`, `yt-clone-processed-media`.
  * Configure CORS if frontend directly fetches media.
  * Set IAM roles so backend/transcoder service account can read/write, and public (or signed URL logic) for video playback.

* **Pub/Sub**

  * Create topic `video-uploaded`.
  * Create subscription for transcoder: either push to Cloud Run endpoint or pull in service.

### 3. IAM & Permissions

* Service accounts for backend and transcoder with least privileges:

  * Firestore User / Datastore User
  * Storage Object Admin (or finer-grained roles per bucket)
  * Pub/Sub Publisher (backend) / Subscriber (transcoder)
* Ensure front-end users authenticate (e.g., via Google Sign-In) if needed; backend verifies JWT.

### 4. Monitoring & Logging

* Use Cloud Logging to capture backend logs, transcoder logs.
* Set up error reporting alerts for failures in transcoding or API errors.
* Monitor Pub/Sub backlog to ensure transcoder keeps up.

---

## 🔧 Configuration & Customization

* **Video Formats & Transcoding Settings**

  * In transcoder service, configure FFmpeg commands: output formats (HLS, DASH), resolutions, bitrates.
  * Thumbnail generation: specify sizes/qualities.

* **API Endpoints**

  * Modify or extend endpoints for comments, likes, subscriptions, etc.
  * Add pagination, search endpoints as needed.

* **Authentication & Authorization**

  * Integrate OAuth2 (Google Sign-In) or custom user management.
  * Protect upload endpoints via JWT; generate short-lived signed URLs for direct-to-Storage uploads if desired.

* **Frontend Theming & UI**

  * Customize UI components, video player skins, dark/light mode.
  * Add analytics/tracking integration (e.g., Google Analytics) for user engagement metrics.

* **Scaling & Performance**

  * Tune Cloud Run concurrency, CPU/memory allocation.
  * Use CDN (Cloud CDN) in front of Storage for faster content delivery.
  * Cache metadata responses (e.g., via CDN or in-memory caching in backend) for popular video pages.

---

## 🤝 Contributing

We welcome contributions! Possible areas:

* **New Features**: comments system, user profiles, subscriptions, notifications.
* **Improved Transcoding**: support more formats, adaptive bitrate streaming.
* **Performance Enhancements**: caching layers, optimized database reads.
* **Testing**: end-to-end tests, load testing for high-concurrency uploads.
* **Documentation**: sample API usage, integration guides, diagrams.

**Contribution Workflow**:

1. Fork the repository.
2. Create a branch: `git checkout -b feature/awesome-feature`.
3. Implement changes, add tests/docs.
4. Submit a Pull Request against `main`. Describe your changes and rationale.
5. Address feedback; ensure CI passes (build, lint, tests).

---

## 🧾 License

Released under the **MIT License**. See [LICENSE](./LICENSE) for details.

---

## 🙋‍♂️ Author

**@deepencoding** – Enthusiast in building high-performance, scalable backend systems with Rust and modern full-stack web architectures. Feel free to open issues or discuss new ideas!
