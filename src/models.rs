use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkConfigMethod {
    DHCP,
    STATIC,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceStatus {
    ONLINE,
    OFFLINE,
    CAMERA_INITIALIZING,
    NO_VIDEO,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnalogAudioOutputSelect {
    DecodeMain,
    DecodeComms,
    DecodeLoop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OperationMode {
    ENCODE,
    DECODE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoOutputMode {
    SDI,
    HDMI,
    LowLatency,
    NormalMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TxProtocolMode {
    Multicast,
    TCP,
    MultiTCP,
    UDP,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PTZSpeed {
    Range0to21,
    Range0to18,
    Range0to7,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BackLightStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExposureMode {
    FULL_AUTO,
    MANUAL,
    SHUTTER_PRI,
    IRIS_PRI,
    BRIGHT,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GainPointStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HighSensitivityStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShutterControlOverwriteStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SlowShutterEnableStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpotlightStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
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
pub enum MatrixStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorSuppressionLevel {
    OFF,
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IRCutFilterStatus {
    Auto,
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NoiseReductionLevel {
    Off,
    Range1to5,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VideoEnhancementStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExternalSettingsStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BandwidthLevel {
    DEFAULT,
    LOW,
    MIDDLE,
    HIGH,
    WIDE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DetailLevel {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CompensationLevel {
    LOW,
    MID,
    HIGH,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StreamingProtocol {
    RTSP,
    SRT,
    HX,
    RTMP,
    DISABLE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NDIDisServStatus {
    NDIDisServEn,
    NDIDisServDis,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    pub firmware_version: String,
    pub format: String,
    pub host_name: String,
    pub ip_address: String,
    pub network_config_method: NetworkConfigMethod,
    pub network_mask: String,
    pub serial_number: String,
    pub status: DeviceStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioSettings {
    pub analog_audio_in_gain: String,
    pub analog_audio_out_gain: String,
    pub analog_audio_output_select: AnalogAudioOutputSelect,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoOutputInterface {
    pub video_output: VideoOutputMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NDISettings {
    pub txpm: TxProtocolMode,
    pub txnetprefix: String,
    pub txnetmask: String,
    pub txmcttl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PTZSettings {
    pub pan_speed: PTZSpeed,
    pub tilt_speed: PTZSpeed,
    pub pan_flip: bool,
    pub tilt_flip: bool,
    pub pan_cw_limit: String,
    pub pan_ccw_limit: String,
    pub tilt_up_limit: String,
    pub tilt_down_limit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExposureSettings {
    pub backlight: BackLightStatus,
    pub exposure_mode: ExposureMode,
    pub exposure_compensation_level: f32,
    pub gain_limit: i32,
    pub gain_point: GainPointStatus,
    pub high_sensitivity: HighSensitivityStatus,
    pub shutter_control_overwrite: ShutterControlOverwriteStatus,
    pub slow_shutter_enable: SlowShutterEnableStatus,
    pub spotlight: SpotlightStatus,
    pub aperature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhiteBalanceSettings {
    pub white_balance_mode: WhiteBalanceMode,
    pub r_gain: i32,
    pub b_gain: i32,
    pub one_push_wb_trigger: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PictureSettings {
    pub brightness: i32,
    pub contrast: i32,
    pub hue: i32,
    pub saturation: i32,
    pub sharpness: i32,
    pub gamma: i32,
    pub white_balance_mode: WhiteBalanceMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColourMatrixSettings {
    pub matrix: MatrixStatus,
    pub level: i32,
    pub color_suppression_level: ColorSuppressionLevel,
    pub ir_cut_filter: IRCutFilterStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedSettings {
    pub nr: NoiseReductionLevel,
    pub ve: VideoEnhancementStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSettings {
    pub external_sync: ExternalSettingsStatus,
    pub external_sync_output: ExternalSettingsStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailSettings {
    pub bandwidth: BandwidthLevel,
    pub detail_level: DetailLevel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GammaSettings {
    pub compensation: CompensationLevel,
    pub gamma: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silicon2CodecSettings {
    pub sil2_codec: i32,
    pub sil2_pkt_size: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silicon2EncodeSettings {
    pub bitrate: i32,
    pub resolution: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NDISourcesMap {
    #[serde(flatten)]
    pub sources: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NDISourceInfo {
    pub source_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EncodeDecodeStatus {
    Encode,
    Decode,
}
impl std::fmt::Display for EncodeDecodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
