use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new().await?;

    let id = std::env::args().nth(1).expect("No channel id provided");

    // Request the channel
    let channel = client.channel(id.parse()?).await?;

    // Print it
    println!("{:#?}", channel);

    // Get its uploads
    let uploads = channel.uploads().await?;

    // Print it
    println!("Uploads: {:#?}", uploads);

    // Get the videos from the uploads playlist
    let videos = uploads.videos();

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
