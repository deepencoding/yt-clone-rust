import Image from "next/image";
import { getVideos } from "./utility/firebase/functions";
import Link from "next/link";
import styles from "./page.module.css";

export default async function Home() {
  const videos = await getVideos();

  return (
    <main>
      {
        videos.map((video) => (
          <Link key={video.filename} href={`/watch?v=${video.filename}`}>
            <Image src={"/thumbnail.png"} alt="video" width={120} height={80} 
              className={styles.thumbnail} />
          </Link>
        ))
      }
    </main>
  );
}

export const revalidate = 30;
