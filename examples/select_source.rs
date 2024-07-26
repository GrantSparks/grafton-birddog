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

    // Get exposure settings
    let exposure_settings = client.get_exposure_settings().await?;
    println!("Exposure Settings: {:?}", exposure_settings);

    // Get white balance settings
    let white_balance_settings = client.get_white_balance_settings().await?;
    println!("White Balance Settings: {:?}", white_balance_settings);

    // Get picture settings
    let picture_settings = client.get_picture_settings().await?;
    println!("Picture Settings: {:?}", picture_settings);

    // Get colour matrix settings
    let colour_matrix_settings = client.get_colour_matrix_settings().await?;
    println!("Colour Matrix Settings: {:?}", colour_matrix_settings);

    // Get advanced settings
    let advanced_settings = client.get_advanced_settings().await?;
    println!("Advanced Settings: {:?}", advanced_settings);

    // Get external settings
    let external_settings = client.get_external_settings().await?;
    println!("External Settings: {:?}", external_settings);

    // Get detail settings
    let detail_settings = client.get_detail_settings().await?;
    println!("Detail Settings: {:?}", detail_settings);

    // Get gamma settings
    let gamma_settings = client.get_gamma_settings().await?;
    println!("Gamma Settings: {:?}", gamma_settings);

    // Get Silicon2 codec settings
    let silicon2_codec_settings = client.get_silicon2_codec_settings().await?;
    println!("Silicon2 Codec Settings: {:?}", silicon2_codec_settings);

    // Get Silicon2 encode settings
    let silicon2_encode_settings = client.get_silicon2_encode_settings().await?;
    println!("Silicon2 Encode Settings: {:?}", silicon2_encode_settings);

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
