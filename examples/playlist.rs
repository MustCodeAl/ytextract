use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new().await?;

    let id = std::env::args().nth(1).expect("No playlist id provided");

    // Request the playlist
    let playlist = client.playlist(id.parse()?).await?;

    // Print it
    println!("{:#?}", playlist);

    // Request the videos
    let videos = playlist.videos();

    futures::pin_mut!(videos);

    // Print them
    println!("Videos: [");

    while let Some(item) = videos.next().await {
        match item {
            Ok(video) => println!("{:#?},", video),
            Err(err) => println!("{:#?},", err),
        }
    }

    println!("]");

    Ok(())
}
