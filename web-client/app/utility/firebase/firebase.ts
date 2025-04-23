// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

import { 
    getAuth,
    signInWithPopup,
    GoogleAuthProvider,
    onAuthStateChanged,
    User
} from "firebase/auth";

// Your web app's Firebase configuration
const firebaseConfig = {
  apiKey: "AIzaSyBNVk-Nzy2_YveIbuUBCeE48InCao9ak3Y",
  authDomain: "yt-clone-rust.firebaseapp.com",
  projectId: "yt-clone-rust",
  appId: "1:543501541359:web:92540b7ba966840ef3601a"
};
import { getFunctions } from "firebase/functions";

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const auth = getAuth(app);

export const functions = getFunctions(undefined, "asia-south1");

/**
 * Signs the user in with Gogle PopUp
 * @returns A Promise that resolves with the user's credentials
 */
export function signInWithGoogle() {
    return signInWithPopup(auth, new GoogleAuthProvider());
}

/**
 * Sign's out the user.
 * @returns A Promise that resolves when the user is signed out.
 */
export function signOut() {
    return auth.signOut();
}

/**
 * Trigger a Callback when user's auth state changes.
 * @returns A function to unsubscribe callback.
 */
export function onAuthStateChangedHelper(callback: (user: User | null) => void) {
    return onAuthStateChanged(auth, callback);
}

