use std::error::Error;

use grafton_birddog::{client::BirdDogClient, models::EncodeDecodeStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with the actual IP address of your BirdDog device.
    let device_ip = "192.168.101.240";
    let base_url = format!("http://{}:8080", device_ip);

    // Create an instance of the BirdDog API client.
    let client = BirdDogClient::new(&base_url);

    // Step 1: Refresh the List of Sources
    client.refresh_ndi_sources_get().await?;
    println!("NDI source list refreshed.");

    // Wait for a few seconds to allow the BirdDog device to update the list of NDI sources.
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Step 2: Retrieve the List of Active NDI Sources
    let sources = client.get_ndi_sources_list().await?;
    println!("Active NDI sources: {:?}", sources);

    // Step 3: Connect to a Specific NDI Source (e.g., CAMERA4)
    let source_name = "SPARTA (Rust NDI Sender)";

    // Check if the specified source exists in the list of active NDI sources.
    if sources.sources.contains_key(source_name) {
        client
            .connect_to_ndi_source(1, source_name, EncodeDecodeStatus::Decode)
            .await?;
        println!("Connected to NDI source: {}", source_name);
    } else {
        println!(
            "Error: NDI source '{}' not found in the active sources list.",
            source_name
        );
        return Err(format!("NDI source '{}' not found.", source_name).into());
    }

    Ok(())
}
