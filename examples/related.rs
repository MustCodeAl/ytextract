use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();

    let id = std::env::args().nth(1).expect("No video id provided");
    let amount = std::env::args().nth(2).expect("No amount provided");

    // Request the video
    let video = client.video(id.parse()?).await?;

    let related = video
        .related()
        .expect("No related videos found")
        .take(amount.parse()?)
        .collect::<Vec<_>>()
        .await;

    println!("{:#?}", related);

    Ok(())
}
