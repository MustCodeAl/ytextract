#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();

    let id = std::env::args().nth(1).expect("No video id provided");

    // Request the streams
    let streams = client.streams(id.parse()?).await?.collect::<Vec<_>>();

    // Print them
    println!("{:#?}", streams);

    Ok(())
}
