use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();

    let id = std::env::args().nth(1).expect("No channel id provided");

    // Request the channel
    let channel = client.channel(id.parse()?).await?;

    // Print it
    println!("{:#?}", channel);

    // Get its uploads
    let uploads = channel.uploads().await?;

    futures::pin_mut!(uploads);

    // Print them
    println!("Uploads: [");

    while let Some(item) = uploads.next().await {
        match item {
            Ok(video) => println!("{:#?},", video),
            Err(err) => println!("{:#?},", err),
        }
    }

    println!("]");

    Ok(())
}
