import * as functions from "firebase-functions/v1";
import {initializeApp} from "firebase-admin/app";
import {getFirestore} from "firebase-admin/firestore";
import * as logger from "firebase-functions/logger";
import {Storage} from "@google-cloud/storage";
import {onCall} from "firebase-functions/v2/https";
const {setGlobalOptions} = require("firebase-functions/v2");

// locate all functions closest to users
setGlobalOptions({region: "asia-south1"});

initializeApp();
const firestore = getFirestore(); // Correct way to initialize Firestore

const storage = new Storage();
const rawVideoBucketName = "yt-raw-videos-deepencoding-clone";

const videosCollectionName = "videos";

export interface Video {
  id?: string,
  uid?: string,
  filename?: string,
  status?: "Processing" | "Processed",
  title?: string,
  description?: string
}

export const createUser = functions.region("asia-south1")
  .auth.user().onCreate((user) => {
    const userInfo = {
      uid: user.uid,
      email: user.email,
      photoUrl: user.photoURL,
    };

    // Save user info to Firestore
    firestore.collection("users").doc(user.uid).set(userInfo);

    logger.info(`User Created: ${JSON.stringify(userInfo)}`);
    return;
  });

export const generateUploadUrl = onCall({maxInstances: 1}, async (request) => {
  if (!request.auth) {
    throw new functions.https.HttpsError(
      "failed-precondition",
      "The function must be called when authenticated."
    );
  }

  const auth = request.auth;
  const data = request.data;
  const bucket = storage.bucket(rawVideoBucketName);

  const fileName = `${auth.uid}-${Date.now()}.${data.fileExtension}`;

  const [url] = await bucket.file(fileName).getSignedUrl({
    version: "v4",
    action: "write",
    expires: Date.now() + 15 * 60 * 1000,
  });

  return {url, fileName};
});

export const getVideos = onCall({maxInstances: 1}, async () => {
  const snapshot = await firestore.collection(videosCollectionName)
    .where("status", "==", "Processed")
    .limit(10).get();
  return snapshot.docs.map((doc) => doc.data());
});
