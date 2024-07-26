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

    // Wait for a few seconds to allow the BirdDog device to update the list of NDI sources.
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Get device info
    let device_info = client.get_device_info().await?;
    println!("Device Info: {:?}", device_info);

    // Get hostname
    let hostname = client.get_hostname().await?;
    println!("Hostname: {}", hostname);

    // Get firmware version
    let firmware_version = client.get_version().await?;
    println!("Firmware Version: {}", firmware_version);

    // Get audio settings
    let audio_settings = client.get_audio_settings().await?;
    println!("Audio Settings: {:?}", audio_settings);

    // Get operation mode
    let operation_mode = client.get_operation_mode().await?;
    println!("Operation Mode: {:?}", operation_mode);

    // Get video output interface
    let video_output_interface = client.get_video_output_interface().await?;
    println!("Video Output Interface: {:?}", video_output_interface);

    // Get NDI settings
    let ndi_settings = client.get_ndi_settings().await?;
    println!("NDI Settings: {:?}", ndi_settings);

    let sources = client.get_ndi_sources_list().await?;
    println!("Active NDI sources: {:?}", sources);

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
