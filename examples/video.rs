#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();

    let id = std::env::args().nth(1).expect("No video id provided");

    // Request the video
    let video = client.video(id.parse()?).await?;

    // Print it
    println!("{:#?}", video);

    Ok(())
}
