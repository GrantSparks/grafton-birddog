use reqwest::Client;

use crate::{
    error::BirdDogError,
    models::{
        AdvancedSettings, AudioSettings, ColourMatrixSettings, DetailSettings, DeviceInfo,
        EncodeDecodeStatus, ExposureSettings, ExternalSettings, GammaSettings, NDISettings,
        NDISourceInfo, NDISourcesMap, OperationMode, PTZSettings, PictureSettings,
        Silicon2CodecSettings, Silicon2EncodeSettings, VideoOutputInterface, WhiteBalanceSettings,
    },
};

/// Client for interacting with the BirdDog REST API.
pub struct BirdDogClient {
    client: Client,
    base_url: String,
}

impl BirdDogClient {
    /// Creates a new instance of `BirdDogClient`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the BirdDog device.
    pub fn new(base_url: &str) -> Self {
        BirdDogClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Fetches basic device information from the endpoint `/about`.
    ///
    /// # Returns
    ///
    /// * `DeviceInfo` - A structure containing the device information.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `DeviceInfo`.
    pub async fn get_device_info(&self) -> Result<DeviceInfo, BirdDogError> {
        let url = format!("{}/about", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<DeviceInfo>()
            .await?;
        Ok(resp)
    }

    /// Retrieves the device hostname from the endpoint `/hostname`.
    ///
    /// # Returns
    ///
    /// * `String` - The hostname of the device.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `String`.
    pub async fn get_hostname(&self) -> Result<String, BirdDogError> {
        let url = format!("{}/hostname", self.base_url);
        let resp = self.client.get(&url).send().await?.text().await?;
        Ok(resp)
    }

    /// Reboots the BirdDog device from the endpoint `/reboot`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn reboot(&self) -> Result<(), BirdDogError> {
        let url = format!("{}/reboot", self.base_url);
        self.client.get(&url).send().await?;
        Ok(())
    }

    /// Restarts the video subsystem on the device from the endpoint `/restart`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn restart(&self) -> Result<(), BirdDogError> {
        let url = format!("{}/restart", self.base_url);
        self.client.get(&url).send().await?;
        Ok(())
    }

    /// Retrieves the hardware version number from the endpoint `/version`.
    ///
    /// # Returns
    ///
    /// * `String` - The hardware version number.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `String`.
    pub async fn get_version(&self) -> Result<String, BirdDogError> {
        let url = format!("{}/version", self.base_url);
        let resp = self.client.get(&url).send().await?.text().await?;
        Ok(resp)
    }

    /// Retrieves audio settings from the endpoint `/analogaudiosetup`.
    ///
    /// # Returns
    ///
    /// * `AudioSettings` - A structure containing the audio settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `AudioSettings`.
    pub async fn get_audio_settings(&self) -> Result<AudioSettings, BirdDogError> {
        let url = format!("{}/analogaudiosetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<AudioSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets audio settings using the endpoint `/analogaudiosetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - A structure containing the audio settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_audio_settings(&self, settings: &AudioSettings) -> Result<(), BirdDogError> {
        let url = format!("{}/analogaudiosetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the current operation mode from the endpoint `/operationmode`.
    ///
    /// # Returns
    ///
    /// * `OperationMode` - The current operation mode of the device.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `OperationMode`.
    pub async fn get_operation_mode(&self) -> Result<OperationMode, BirdDogError> {
        let url = format!("{}/operationmode", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<OperationMode>()
            .await?;
        Ok(resp)
    }

    /// Sets the operation mode using the endpoint `/operationmode`.
    ///
    /// # Arguments
    ///
    /// * `mode` - The operation mode to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_operation_mode(&self, mode: &OperationMode) -> Result<(), BirdDogError> {
        let url = format!("{}/operationmode", self.base_url);
        self.client.post(&url).json(mode).send().await?;
        Ok(())
    }

    /// Retrieves the current video output interface mode from the endpoint `/videooutputinterface`.
    ///
    /// # Returns
    ///
    /// * `VideoOutputInterface` - The current video output interface mode.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `VideoOutputInterface`.
    pub async fn get_video_output_interface(&self) -> Result<VideoOutputInterface, BirdDogError> {
        let url = format!("{}/videooutputinterface", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<VideoOutputInterface>()
            .await?;
        Ok(resp)
    }

    /// Sets the video output interface mode using the endpoint `/videooutputinterface`.
    ///
    /// # Arguments
    ///
    /// * `interface` - The video output interface mode to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_video_output_interface(
        &self,
        interface: &VideoOutputInterface,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/videooutputinterface", self.base_url);
        self.client.post(&url).json(interface).send().await?;
        Ok(())
    }

    /// Retrieves the current NDI network settings from the endpoint `/encodeTransport`.
    ///
    /// # Returns
    ///
    /// * `NDISettings` - The current NDI network settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `NDISettings`.
    pub async fn get_ndi_settings(&self) -> Result<NDISettings, BirdDogError> {
        let url = format!("{}/encodeTransport", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<NDISettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the NDI network settings using the endpoint `/encodeTransport`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The NDI network settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_ndi_settings(&self, settings: &NDISettings) -> Result<(), BirdDogError> {
        let url = format!("{}/encodeTransport", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the PTZ settings from the endpoint `/birddogptzsetup`.
    ///
    /// # Returns
    ///
    /// * `PTZSettings` - The PTZ settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `PTZSettings`.
    pub async fn get_ptz_settings(&self) -> Result<PTZSettings, BirdDogError> {
        let url = format!("{}/birddogptzsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<PTZSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the PTZ settings using the endpoint `/birddogptzsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The PTZ settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_ptz_settings(&self, settings: &PTZSettings) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogptzsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the exposure settings from the endpoint `/birddogexpsetup`.
    ///
    /// # Returns
    ///
    /// * `ExposureSettings` - The exposure settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `ExposureSettings`.
    pub async fn get_exposure_settings(&self) -> Result<ExposureSettings, BirdDogError> {
        let url = format!("{}/birddogexpsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ExposureSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the exposure settings using the endpoint `/birddogexpsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The exposure settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_exposure_settings(
        &self,
        settings: &ExposureSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogexpsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the white balance settings from the endpoint `/birddogwbsetup`.
    ///
    /// # Returns
    ///
    /// * `WhiteBalanceSettings` - The white balance settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `WhiteBalanceSettings`.
    pub async fn get_white_balance_settings(&self) -> Result<WhiteBalanceSettings, BirdDogError> {
        let url = format!("{}/birddogwbsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<WhiteBalanceSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the white balance settings using the endpoint `/birddogwbsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The white balance settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_white_balance_settings(
        &self,
        settings: &WhiteBalanceSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogwbsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the picture settings from the endpoint `/birddogpicsetup`.
    ///
    /// # Returns
    ///
    /// * `PictureSettings` - The picture settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `PictureSettings`.
    pub async fn get_picture_settings(&self) -> Result<PictureSettings, BirdDogError> {
        let url = format!("{}/birddogpicsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<PictureSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the picture settings using the endpoint `/birddogpicsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The picture settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_picture_settings(
        &self,
        settings: &PictureSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogpicsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the color matrix settings from the endpoint `/birddogcmsetup`.
    ///
    /// # Returns
    ///
    /// * `ColourMatrixSettings` - The color matrix settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `ColourMatrixSettings`.
    pub async fn get_colour_matrix_settings(&self) -> Result<ColourMatrixSettings, BirdDogError> {
        let url = format!("{}/birddogcmsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ColourMatrixSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the color matrix settings using the endpoint `/birddogcmsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The color matrix settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_colour_matrix_settings(
        &self,
        settings: &ColourMatrixSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogcmsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the advanced settings from the endpoint `/birddogadvancesetup`.
    ///
    /// # Returns
    ///
    /// * `AdvancedSettings` - The advanced settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `AdvancedSettings`.
    pub async fn get_advanced_settings(&self) -> Result<AdvancedSettings, BirdDogError> {
        let url = format!("{}/birddogadvancesetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<AdvancedSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the advanced settings using the endpoint `/birddogadvancesetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The advanced settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_advanced_settings(
        &self,
        settings: &AdvancedSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogadvancesetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the external settings from the endpoint `/birddogexternalsetup`.
    ///
    /// # Returns
    ///
    /// * `ExternalSettings` - The external settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `ExternalSettings`.
    pub async fn get_external_settings(&self) -> Result<ExternalSettings, BirdDogError> {
        let url = format!("{}/birddogexternalsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ExternalSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the external settings using the endpoint `/birddogexternalsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The external settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_external_settings(
        &self,
        settings: &ExternalSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogexternalsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the detail settings from the endpoint `/birddogdetsetup`.
    ///
    /// # Returns
    ///
    /// * `DetailSettings` - The detail settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `DetailSettings`.
    pub async fn get_detail_settings(&self) -> Result<DetailSettings, BirdDogError> {
        let url = format!("{}/birddogdetsetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<DetailSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the detail settings using the endpoint `/birddogdetsetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The detail settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_detail_settings(&self, settings: &DetailSettings) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogdetsetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the gamma settings from the endpoint `/birddoggammasetup`.
    ///
    /// # Returns
    ///
    /// * `GammaSettings` - The gamma settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `GammaSettings`.
    pub async fn get_gamma_settings(&self) -> Result<GammaSettings, BirdDogError> {
        let url = format!("{}/birddoggammasetup", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<GammaSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the gamma settings using the endpoint `/birddoggammasetup`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The gamma settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_gamma_settings(&self, settings: &GammaSettings) -> Result<(), BirdDogError> {
        let url = format!("{}/birddoggammasetup", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the Silicon2 codec settings from the endpoint `/birddogsil2codec`.
    ///
    /// # Returns
    ///
    /// * `Silicon2CodecSettings` - The Silicon2 codec settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `Silicon2CodecSettings`.
    pub async fn get_silicon2_codec_settings(&self) -> Result<Silicon2CodecSettings, BirdDogError> {
        let url = format!("{}/birddogsil2codec", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Silicon2CodecSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the Silicon2 codec settings using the endpoint `/birddogsil2codec`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The Silicon2 codec settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_silicon2_codec_settings(
        &self,
        settings: &Silicon2CodecSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogsil2codec", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the Silicon2 encode settings from the endpoint `/birddogsil2enc`.
    ///
    /// # Returns
    ///
    /// * `Silicon2EncodeSettings` - The Silicon2 encode settings.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `Silicon2EncodeSettings`.
    pub async fn get_silicon2_encode_settings(
        &self,
    ) -> Result<Silicon2EncodeSettings, BirdDogError> {
        let url = format!("{}/birddogsil2enc", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Silicon2EncodeSettings>()
            .await?;
        Ok(resp)
    }

    /// Sets the Silicon2 encode settings using the endpoint `/birddogsil2enc`.
    ///
    /// # Arguments
    ///
    /// * `settings` - The Silicon2 encode settings to be set.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn set_silicon2_encode_settings(
        &self,
        settings: &Silicon2EncodeSettings,
    ) -> Result<(), BirdDogError> {
        let url = format!("{}/birddogsil2enc", self.base_url);
        self.client.post(&url).json(settings).send().await?;
        Ok(())
    }

    /// Retrieves the list of active NDI sources on the network from the endpoint `/List`.
    ///
    /// # Returns
    ///
    /// * `NDISourcesMap` - The map of active NDI sources on the network.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `NDISourcesMap`.
    pub async fn get_ndi_sources_list(&self) -> Result<NDISourcesMap, BirdDogError> {
        let url = format!("{}/List", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let raw_body = response.text().await?;
            let ndi_sources_map = serde_json::from_str::<NDISourcesMap>(&raw_body)
                .map_err(BirdDogError::SerializationError)?;
            Ok(ndi_sources_map)
        } else {
            let error_text = response.text().await?;
            Err(BirdDogError::RequestError(error_text))
        }
    }

    /// Refreshes the NDI source list using the GET method on the endpoint `/refresh`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn refresh_ndi_sources_get(&self) -> Result<(), BirdDogError> {
        let url = format!("{}/refresh", self.base_url);
        self.client.get(&url).send().await?;
        Ok(())
    }

    /// Refreshes the NDI source list using the POST method on the endpoint `/refresh`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn refresh_ndi_sources_post(&self) -> Result<(), BirdDogError> {
        let url = format!("{}/refresh", self.base_url);
        self.client.post(&url).send().await?;
        Ok(())
    }

    /// Resets the NDI source list using the GET method on the endpoint `/reset`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn reset_ndi_sources_get(&self) -> Result<(), BirdDogError> {
        let url = format!("{}/reset", self.base_url);
        self.client.get(&url).send().await?;
        Ok(())
    }

    /// Resets the NDI source list using the POST method on the endpoint `/reset`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn reset_ndi_sources_post(&self) -> Result<(), BirdDogError> {
        let url = format!("{}/reset", self.base_url);
        self.client.post(&url).send().await?;
        Ok(())
    }

    /// Captures the frame for encode/decode from the endpoint `/capture`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn capture_frame(
        &self,
        ch_num: u8,
        status: EncodeDecodeStatus,
    ) -> Result<(), BirdDogError> {
        let url = format!(
            "{}/capture?ChNum={}&status={}",
            self.base_url, ch_num, status
        );
        self.client.get(&url).send().await?;
        Ok(())
    }

    /// Retrieves the connected NDI source info from the endpoint `/connectTo`.
    ///
    /// # Returns
    ///
    /// * `NDISourceInfo` - The connected NDI source info.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails or the response cannot be parsed as `NDISourceInfo`.
    pub async fn get_connected_ndi_source(
        &self,
        ch_num: u8,
        status: EncodeDecodeStatus,
    ) -> Result<NDISourceInfo, BirdDogError> {
        let url = format!(
            "{}/connectTo?ChNum={}&status={}",
            self.base_url, ch_num, status
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<NDISourceInfo>()
            .await?;
        Ok(resp)
    }

    /// Connects to an NDI source using the endpoint `/connectTo`.
    ///
    /// # Errors
    ///
    /// Returns a `BirdDogError` if the request fails.
    pub async fn connect_to_ndi_source(
        &self,
        ch_num: u8,
        source_name: &str,
        status: EncodeDecodeStatus,
    ) -> Result<(), BirdDogError> {
        let url = format!(
            "{}/connectTo?ChNum={}&sourcename={}&status={}",
            self.base_url, ch_num, source_name, status
        );
        self.client.post(&url).send().await?;
        Ok(())
    }
}
