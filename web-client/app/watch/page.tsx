"use client";

import { useSearchParams } from "next/navigation";
import { Suspense } from 'react'

function Search() {
  const videoPrefix = "https://storage.googleapis.com/yt-processed-videos-deepencoding-clone/";
  const videoSrc = useSearchParams().get('v');
 
  return <video controls src={videoPrefix + videoSrc} />
}

export default function Watch() {
    return (
        <div>
            <h1>Watch Page</h1>
            <Suspense>
                <Search />
            </Suspense>
        </div>
    );
}