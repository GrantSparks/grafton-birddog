use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Enums with lowercase renaming
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NetworkConfigMethod {
    DHCP,
    STATIC,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DeviceStatus {
    ONLINE,
    OFFLINE,
    CAMERA_INITIALIZING,
    NO_VIDEO,
    ACTIVE,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum AnalogAudioOutputSelect {
    DecodeMain,
    DecodeComms,
    DecodeLoop,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OperationMode {
    ENCODE,
    DECODE,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum VideoOutputMode {
    SDI,
    HDMI,
    LowLatency,
    NormalMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum TxProtocolMode {
    Multicast,
    TCP,
    MultiTCP,
    UDP,
}

// Enums with special renaming
#[derive(Serialize, Deserialize, Debug)]
pub enum PTZSpeed {
    #[serde(rename = "Range0to21")]
    Range0to21,
    #[serde(rename = "Range0to18")]
    Range0to18,
    #[serde(rename = "Range0to7")]
    Range0to7,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum BackLightStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ExposureMode {
    FULL_AUTO,
    MANUAL,
    SHUTTER_PRI,
    IRIS_PRI,
    BRIGHT,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum GainPointStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum HighSensitivityStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ShutterControlOverwriteStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SlowShutterEnableStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum SpotlightStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum WhiteBalanceMode {
    AUTO,
    INDOOR,
    OUTDOOR,
    OUTDOOR_AUTO,
    ONEPUSH,
    ATW,
    MANUAL,
    SVL_AUTO,
    SVL,
    SVL_OUTDOOR_AUTO,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum MatrixStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ColorSuppressionLevel {
    OFF,
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum IRCutFilterStatus {
    Auto,
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum NoiseReductionLevel {
    Off,
    Range1to5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum VideoEnhancementStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ExternalSettingsStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum BandwidthLevel {
    DEFAULT,
    LOW,
    MIDDLE,
    HIGH,
    WIDE,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum DetailLevel {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum CompensationLevel {
    LOW,
    MID,
    HIGH,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StreamingProtocol {
    RTSP,
    SRT,
    HX,
    RTMP,
    DISABLE,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum NDIDisServStatus {
    NDIDisServEn,
    NDIDisServDis,
}

// Structs with specific renaming
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    #[serde(rename = "FallbackIP")]
    pub fallback_ip: String,
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "GateWay")]
    pub gateway: String,
    #[serde(rename = "HardwareVersion")]
    pub hardware_version: String,
    #[serde(rename = "HostName")]
    pub host_name: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "MCUVersion")]
    pub mcu_version: String,
    #[serde(rename = "NetworkConfigMethod")]
    pub network_config_method: NetworkConfigMethod,
    #[serde(rename = "NetworkMask")]
    pub network_mask: String,
    #[serde(rename = "SerialNumber")]
    pub serial_number: String,
    #[serde(rename = "Status")]
    pub status: DeviceStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioSettings {
    #[serde(rename = "AnalogAudioInGain")]
    pub analog_audio_in_gain: String,
    #[serde(rename = "AnalogAudioOutGain")]
    pub analog_audio_out_gain: String,
    #[serde(rename = "AnalogAudiooutputselect")]
    pub analog_audio_output_select: AnalogAudioOutputSelect,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoOutputInterface {
    SDI,
    HDMI,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NDISettings {
    #[serde(rename = "Txpm")]
    pub txpm: TxProtocolMode,
    #[serde(rename = "Txnetprefix")]
    pub txnetprefix: String,
    #[serde(rename = "Txnetmask")]
    pub txnetmask: String,
    #[serde(rename = "Txmcttl")]
    pub txmcttl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PTZSettings {
    #[serde(rename = "PanSpeed")]
    pub pan_speed: String,
    #[serde(rename = "TiltSpeed")]
    pub tilt_speed: String,
    #[serde(rename = "ZoomSpeed")]
    pub zoom_speed: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExposureSettings {
    #[serde(rename = "BackLight")]
    pub backlight: BackLightStatus,
    #[serde(rename = "ExpMode")]
    pub exposure_mode: ExposureMode,
    #[serde(rename = "ExpCompLvl")]
    pub exposure_compensation_level: f32,
    #[serde(rename = "GainLimit")]
    pub gain_limit: i32,
    #[serde(rename = "GainPoint")]
    pub gain_point: GainPointStatus,
    #[serde(rename = "HighSensitivity")]
    pub high_sensitivity: HighSensitivityStatus,
    #[serde(rename = "ShutterControlOverwrite")]
    pub shutter_control_overwrite: ShutterControlOverwriteStatus,
    #[serde(rename = "SlowShutterEn")]
    pub slow_shutter_enable: SlowShutterEnableStatus,
    #[serde(rename = "Spotlight")]
    pub spotlight: SpotlightStatus,
    #[serde(rename = "IrisLevel")]
    pub aperature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhiteBalanceSettings {
    #[serde(rename = "WbMode")]
    pub white_balance_mode: WhiteBalanceMode,
    #[serde(rename = "RG")]
    pub r_gain: i32,
    #[serde(rename = "BG")]
    pub b_gain: i32,
    #[serde(rename = "OnePushWb")]
    pub one_push_wb_trigger: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PictureSettings {
    #[serde(rename = "Brightness")]
    pub brightness: i32,
    #[serde(rename = "Contrast")]
    pub contrast: i32,
    #[serde(rename = "Hue")]
    pub hue: i32,
    #[serde(rename = "Saturation")]
    pub saturation: i32,
    #[serde(rename = "Sharpness")]
    pub sharpness: i32,
    #[serde(rename = "Gamma")]
    pub gamma: i32,
    #[serde(rename = "WbMode")]
    pub white_balance_mode: WhiteBalanceMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColourMatrixSettings {
    #[serde(rename = "Matrix")]
    pub matrix: MatrixStatus,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "ChromeSuppress")]
    pub color_suppression_level: ColorSuppressionLevel,
    #[serde(rename = "IRCutFilter")]
    pub ir_cut_filter: IRCutFilterStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedSettings {
    #[serde(rename = "TWODNR")]
    pub nr: NoiseReductionLevel,
    #[serde(rename = "VideoEnhancement")]
    pub ve: VideoEnhancementStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSettings {
    #[serde(rename = "Aux")]
    pub external_sync: ExternalSettingsStatus,
    #[serde(rename = "V12vOut")]
    pub external_sync_output: ExternalSettingsStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailSettings {
    #[serde(rename = "Bandwidth")]
    pub bandwidth: BandwidthLevel,
    #[serde(rename = "Detail")]
    pub detail_level: DetailLevel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GammaSettings {
    #[serde(rename = "BrightnessComp")]
    pub compensation: CompensationLevel,
    #[serde(rename = "BlackGammaLevel")]
    pub gamma: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silicon2CodecSettings {
    #[serde(rename = "Bitrate")]
    pub bitrate: i32,
    #[serde(rename = "QuantFactorI")]
    pub quant_factor_i: i32,
    #[serde(rename = "QuantFactorP")]
    pub quant_factor_p: i32,
    #[serde(rename = "GOPSize")]
    pub gop_size: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silicon2EncodeSettings {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "Protocol")]
    pub protocol: StreamingProtocol,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "Port")]
    pub port: i32,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "Latency")]
    pub latency: i32,
    #[serde(rename = "Encryption")]
    pub encryption: bool,
    #[serde(rename = "Passphrase")]
    pub passphrase: String,
    #[serde(rename = "PBKeyLen")]
    pub pb_key_len: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NDISourcesMap {
    #[serde(flatten)]
    pub sources: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NDISourceInfo {
    #[serde(rename = "sourceName")]
    pub source_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EncodeDecodeStatus {
    Encode,
    Decode,
}

impl std::fmt::Display for EncodeDecodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub errno: Option<i32>,
    pub code: Option<String>,
    pub syscall: Option<String>,
    pub path: Option<String>,
    pub status: Option<String>,
}
