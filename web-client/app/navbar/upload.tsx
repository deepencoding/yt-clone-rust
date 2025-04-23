"use client";

import React, { Fragment } from "react";
import { uploadVideo } from "../utility/firebase/functions";

import styles from "./upload.module.css";

export default function Upload() {
    const handleUpload = async (file: File) => {
        try {
            const response = await uploadVideo(file);
            alert(`File uploaded succesfully. Response: ${JSON.stringify(response)}`);
        } catch (e) {
            alert(`Failed to upload file: ${e}`);
        }
    };

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.item(0);
        if (file) {
            handleUpload(file);
        }
    };

    return (
        <Fragment>
            <input id="upload" className={styles.uploadInput} type="file" accept="video/*" 
                onChange={handleFileChange}
            />
            <label htmlFor="upload" className={styles.uploadButton}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.2} stroke="currentColor" className="size-6">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
            </label>
        </Fragment>
    );
}
