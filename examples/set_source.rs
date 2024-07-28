use grafton_birddog::{client::BirdDogClient, models::EncodeDecodeStatus};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with the actual IP address of your BirdDog device.
    let device_ip = "192.168.101.240";
    let base_url = format!("http://{}:8080", device_ip);

    // Create an instance of the BirdDog API client.
    let client = BirdDogClient::new(&base_url);

    client.refresh_ndi_sources_get().await?;
    println!("NDI source list refreshed.");

    let sources = client.get_ndi_sources_list().await?;
    println!("Active NDI sources: {:?}", sources);

    // Connect to the first NDI source in the list.
    if let Some((source_name, _)) = sources.sources.iter().next() {
        client
            .connect_to_ndi_source(1, source_name, EncodeDecodeStatus::Decode)
            .await?;
        println!("Connected to NDI source: {}", source_name);
    } else {
        println!("Error: No NDI sources found.");
    }

    Ok(())
}
