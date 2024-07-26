use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NetworkConfigMethod {
    Dhcp,
    Static,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DeviceStatus {
    Online,
    Offline,
    CameraInitializing,
    NoVideo,
    Active,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum AudioOutputSelect {
    DecodeMain,
    DecodeComms,
    DecodeLoop,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OperationMode {
    Encode,
    Decode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum VideoOutputMode {
    Sdi,
    Hdmi,
    LowLatency,
    NormalMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TxProtocolMode {
    Multicast,
    Tcp,
    MultiTcp,
    Udp,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum OnOffStatus {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ExposureMode {
    FullAuto,
    Manual,
    ShutterPri,
    IrisPri,
    Bright,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum WhiteBalanceMode {
    Auto,
    Indoor,
    Outdoor,
    OutdoorAuto,
    OnePush,
    Atw,
    Manual,
    SvlAuto,
    Svl,
    SvlOutdoorAuto,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ColorSuppressionLevel {
    Off,
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum NoiseReductionLevel {
    Off,
    Range1to5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum BandwidthLevel {
    Default,
    Low,
    Middle,
    High,
    Wide,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum BrightnessComp {
    VeryDark,
    Dark,
    Standard,
    Bright,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum CompensationLevel {
    Low,
    Mid,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum NdiDisServStatus {
    NdiDisServEn,
    NdiDisServDis,
}

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
    pub analog_audio_output_select: AudioOutputSelect,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoOutputInterface {
    Sdi,
    Hdmi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NdiSettings {
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
pub struct PtzSettings {
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
    pub backlight: OnOffStatus,
    #[serde(rename = "ExpMode")]
    pub exposure_mode: ExposureMode,
    #[serde(rename = "ExpCompLvl")]
    pub exposure_compensation_level: f32,
    #[serde(rename = "GainLimit")]
    pub gain_limit: i32,
    #[serde(rename = "GainPoint")]
    pub gain_point: OnOffStatus,
    #[serde(rename = "HighSensitivity")]
    pub high_sensitivity: OnOffStatus,
    #[serde(rename = "ShutterControlOverwrite")]
    pub shutter_control_overwrite: OnOffStatus,
    #[serde(rename = "SlowShutterEn")]
    pub slow_shutter_enable: OnOffStatus,
    #[serde(rename = "Spotlight")]
    pub spotlight: OnOffStatus,
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
    pub matrix: OnOffStatus,
    #[serde(rename = "Level")]
    pub level: i32,
    #[serde(rename = "ChromeSuppress")]
    pub color_suppression_level: ColorSuppressionLevel,
    #[serde(rename = "IRCutFilter")]
    pub ir_cut_filter: OnOffStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedSettings {
    #[serde(rename = "Brightness")]
    pub brightness: i32, // Range (0 to 6)
    #[serde(rename = "BrightnessComp")]
    pub brightness_comp: BrightnessComp, // VERY DARK, DARK, STANDARD, BRIGHT
    #[serde(rename = "CompLevel")]
    pub comp_level: CompensationLevel, // LOW, MID, HIGH
    #[serde(rename = "GammaOffset")]
    pub gamma_offset: i32, // Range (16 to 64)
    #[serde(rename = "HighResolution")]
    pub high_resolution: OnOffStatus, // On, Off
    #[serde(rename = "VideoEnhancement")]
    pub video_enhancement: OnOffStatus, // On, Off
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalSettings {
    #[serde(rename = "Aux")]
    pub aux: OnOffStatus,
    #[serde(rename = "RainWiper")]
    pub rain_wiper: OnOffStatus,
    #[serde(rename = "V12vOut")]
    pub v12v_out: OnOffStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum BwBalanceType {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailSettings {
    #[serde(rename = "Bandwidth")]
    pub bandwidth: BandwidthLevel,
    #[serde(rename = "BwBalance")]
    pub bw_balance: BwBalanceType,
    #[serde(rename = "Crispening")]
    pub crispening: u8,
    #[serde(rename = "Detail")]
    pub detail: OnOffStatus,
    #[serde(rename = "HighLightDetail")]
    pub highlight_detail: u8,
    #[serde(rename = "HvBalance")]
    pub hv_balance: i8,
    #[serde(rename = "Limit")]
    pub limit: u8,
    #[serde(rename = "SuperLow")]
    pub super_low: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum BlackLevelRange {
    Low,
    Mid,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SettingsType {
    Pattern,
    Standard,
    Straight,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GammaSettings {
    #[serde(rename = "BlackGammaLevel")]
    pub black_gamma_level: u8,
    #[serde(rename = "BlackLevel")]
    pub black_level: u8,
    #[serde(rename = "BlackLevelRange")]
    pub black_level_range: BlackLevelRange,
    #[serde(rename = "Effect")]
    pub effect: i8,
    #[serde(rename = "Level")]
    pub level: u8,
    #[serde(rename = "Offset")]
    pub offset: i8,
    #[serde(rename = "Pattern")]
    pub pattern: u16,
    #[serde(rename = "PatternFine")]
    pub pattern_fine: u8,
    #[serde(rename = "Settings")]
    pub settings: SettingsType,
    #[serde(rename = "VisibilityEnhancer")]
    pub visibility_enhancer: OnOffStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    Rtsp,
    Srt,
    Hx,
    Rtmp,
    Disable,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ModeSel {
    Custom,
    Low,
    Med,
    High,
    Ultra,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum BitrateControl {
    Vbr,
    Cbr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodecModeSettings {
    #[serde(rename = "Bitrate")]
    pub bitrate: i32,
    #[serde(rename = "GOPSize")]
    pub gop_size: i32,
    #[serde(rename = "QuantFactorI")]
    pub quant_factor_i: i32,
    #[serde(rename = "QuantFactorP")]
    pub quant_factor_p: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodecSettings {
    #[serde(rename = "BitrateControl")]
    pub bitrate_control: BitrateControl,
    #[serde(rename = "ModeSel")]
    pub mode_sel: ModeSel,
    #[serde(rename = "Custom")]
    pub custom: CodecModeSettings,
    #[serde(rename = "Low")]
    pub low: CodecModeSettings,
    #[serde(rename = "Med")]
    pub med: CodecModeSettings,
    #[serde(rename = "High")]
    pub high: CodecModeSettings,
    #[serde(rename = "Ultra")]
    pub ultra: CodecModeSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silicon2CodecSettings {
    #[serde(rename = "DISABLE")]
    pub disable: CodecSettings,
    #[serde(rename = "HX")]
    pub hx: CodecSettings,
    #[serde(rename = "RTMP")]
    pub rtmp: CodecSettings,
    #[serde(rename = "RTSP")]
    pub rtsp: CodecSettings,
    #[serde(rename = "SRT")]
    pub srt: CodecSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum AuthEnable {
    True,
    False,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Encryption {
    True,
    False,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Mode {
    Caller,
    Listener,
    Rendezvous,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ServerSelection {
    Local,
    Remote,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RTMPSettings {
    #[serde(rename = "AuthEnable")]
    pub auth_enable: AuthEnable,
    #[serde(rename = "ConnectionURL")]
    pub connection_url: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Server")]
    pub server: String,
    #[serde(rename = "ServerSelection")]
    pub server_selection: ServerSelection,
    #[serde(rename = "StreamKeyLocal")]
    pub stream_key_local: String,
    #[serde(rename = "StreamKeyRemote")]
    pub stream_key_remote: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RTSPSettings {
    #[serde(rename = "AuthEnable")]
    pub auth_enable: AuthEnable,
    #[serde(rename = "ConnectionURL")]
    pub connection_url: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Port")]
    pub port: i32,
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "UserName")]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SRTSettings {
    #[serde(rename = "ConnectionURL")]
    pub connection_url: String,
    #[serde(rename = "Encryption")]
    pub encryption: Encryption,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "Port")]
    pub port: i32,
    #[serde(rename = "Latency")]
    pub latency: i32,
    #[serde(rename = "Mode")]
    pub mode: Mode,
    #[serde(rename = "Passphrase")]
    pub passphrase: String,
    #[serde(rename = "PBKeyLen")]
    pub pb_key_len: i32,
    #[serde(rename = "StreamID")]
    pub stream_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Silicon2EncodeSettings {
    #[serde(rename = "HaiVisionPlayerSupport")]
    pub hai_vision_player_support: bool,
    #[serde(rename = "RTMP")]
    pub rtmp: RTMPSettings,
    #[serde(rename = "RTSP")]
    pub rtsp: RTSPSettings,
    #[serde(rename = "SRT")]
    pub srt: SRTSettings,
    #[serde(rename = "StreamingProtocol")]
    pub streaming_protocol: Protocol,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NdiSourcesMap {
    #[serde(flatten)]
    pub sources: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NdiSourceInfo {
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
