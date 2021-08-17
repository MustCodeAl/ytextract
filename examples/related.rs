use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new().await?;

    let id = std::env::args().nth(1).expect("No video id provided");
    let amount = std::env::args().nth(2).expect("No amount provided");

    // Request the video
    let video = client.video(id.parse()?).await?;

    let related = video
        .related()
        .take(amount.parse()?)
        .collect::<Vec<_>>()
        .await;

    println!("{:#?}", related);

    Ok(())
}
