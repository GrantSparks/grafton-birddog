use grafton_birddog::{apis::configuration::Configuration, apis::default_api::list_get};

#[tokio::main]
async fn main() {
    // Create a new Configuration object
    let mut config = Configuration::new();

    // Set the base_path to the desired URL
    config.base_path = "http://192.168.101.240:8080".to_owned();

    // Optionally, print the configuration to verify
    println!("{:?}", config);

    // Call the list_get function to get the list of sources
    match list_get(&config).await {
        Ok(_) => println!("Successfully fetched the list."),
        Err(e) => eprintln!("Error fetching the list: {:?}", e),
    }
}
