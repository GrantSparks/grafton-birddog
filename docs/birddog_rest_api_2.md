# BirdDog RESTful API 2.0

## API and SDK Documentation

**Version:** 2.0.0

This is a sample BirdDog API server. You can find out more about BirdDog Products at [http://www.bird-dog.tv](http://www.bird-dog.tv).

## BasicDeviceInformation

### aboutGet

Retrieve basic information from your BirdDog device

**Endpoint:** `/about`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/about?FirmwareVersion="
```

**Parameters:**

- **Header parameters:**
  - `Format` (string): Device manufacturer extended `BIRDDOG`
  - `HostName` (string): Hardware Revision `1.0`
  - `IPAddress` (string): Device IP Address `xxx.xxx.xxx.xxx`
  - `NetworkConfigMethod` (string): IP Address method `DHCP` `STATIC`
  - `SerialNumber` (string): Device Hostname (AVAHI) `birddog-nnnnn`
  - `Status` (string): Current device status `ONLINE` `OFFLINE` `CAMERA INITIALIZING` `NO VIDEO`
- **Query parameters:**
  - `FirmwareVersion` (string): Device manufacturer `BIRDDOG`

**Responses:**

- **Status:** default - Device standard response
- **Example data:**
  ```json
  {
    "FirmwareVersion": "BirdDog Firmware version",
    "Format": "CAM 1",
    "HostName": "birddog-device",
    "IPAddress": "192.168.100.100",
    "NetworkConfigMethod": "dhcp",
    "NetworkMask": "255.255.255.0",
    "SerialNumber": "0123456789",
    "Status": "active"
  }
  ```

### hostnameGet

Retrieve device hostname

This command will return a text string with the hostname of the BirdDog device.

**Endpoint:** `/hostname`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/hostname/"
```

**Responses:**

- **Status:** default - Device hostname (NDI Hostname)
- **Example data:**
  ```text
  birddog-nnnnn
  ```

### rebootGet

Reboot BirdDog device

This command will immediately initiate a reboot of the BirdDog device.

**Endpoint:** `/reboot`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/reboot"
```

**Responses:**

- **Status:** default - Immediately upon device reboot, the below value will be returned
- **Example data:**
  ```json
  {}
  ```

### rebootPost

Reboot BirdDog device

This command will immediately initiate a reboot of the BirdDog device.

**Endpoint:** `/reboot`

**Curl Example:**

```bash
curl -X POST\
-H "Accept: application/json"\
"http://192.168.2.197:8080/reboot"
```

**Responses:**

- **Status:** default - Immediately upon device reboot, the below value will be returned
- **Example data:**
  ```json
  {}
  ```

### restartGet

Restart video subsystem on device

This command will initiate a video system restart on the BirdDog device.

**Endpoint:** `/restart`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/restart"
```

**Responses:**

- **Status:** default - Immediately upon restart video, the below value will be returned
- **Example data:**
  ```json
  {}
  ```

### restartPost

Restart video subsystem on device

This command will initiate a video system restart on the BirdDog device.

**Endpoint:** `/restart`

**Curl Example:**

```bash
curl -X POST\
-H "Accept: application/json"\
"http://192.168.2.197:8080/restart"
```

**Responses:**

- **Status:** default - Immediately upon restart video, the below value will be returned
- **Example data:**
  ```json
  {}
  ```

### versionGet

Hardware version number query

This command will recall the BirdDog hardware identifier, this includes the model number as well as the hardware version.

**Endpoint:** `/version`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/version"
```

**Responses:**

- **Status:** default - Device standard response
- **Example data:**
  ```text
  BirdDog P200A4_A5
  ```

## DEVICESETTINGS

### analogaudiosetupGet

Retrieve Audio Settings

AnalogAudiooutputselect is not present in Flex Encode, WP Encode, QUAD, P200/A200/A300, P100/PF120 and P4k/P400

**Endpoint:** `/analogaudiosetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/analogaudiosetup?AnalogAudioInGain=&AnalogAudioOutGain=&AnalogAudiooutputselect="
```

**Parameters:**

- **Query parameters:**
  - `AnalogAudioInGain` (string): Audio in gain Range 0 to 100
  - `AnalogAudioOutGain` (string): Audio out gain Range 0 to 100
  - `AnalogAudiooutputselect` (string): DecodeMain, DecodeComms, DecodeLoop

**Responses:**

- **Status:** default - Device standard response
- **Example data:**
  ```json
  {
    "AnalogAudioInGain": "80",
    "AnalogAudioOutGain": "80",
    "AnalogAudiooutputselect": "DecodeMain"
  }
  ```

### analogaudiosetupPost

Set Audio Settings

AnalogAudiooutputselect is not present in Flex Encode, WP Encode, QUAD, P200/A200/A300, P100/PF120 and P4k/P400

**Endpoint:** `/analogaudiosetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"AnalogAudioInGain":"50","AnalogAudioOutGain":"50","AnalogAudiooutputselect":"DecodeMain"}' \
http://192.168.2.197:8080/analogaudiosetup
```

**Parameters:**

- **Query parameters:**
  - `AnalogAudioInGain` (string): Audio in gain Range 0 to 100
  - `AnalogAudioOutGain` (string): Audio out gain Range 0 to 100
  - `AnalogAudiooutputselect` (string): DecodeMain, DecodeComms, DecodeLoop

**Responses:**

- **Status:** default - Device standard response
- **Example data:**
  ```json
  {
    "AnalogAudioInGain": "80",
    "AnalogAudioOutGain": "80",
    "AnalogAudiooutputselect": "DecodeMain"
  }
  ```

### operationmodeGet

Retrieve current operation mode of your BirdDog device (encode/decode)

This shows you what mode your BirdDog device is operating in - NDI(R) encode or NDI(R) decode.

Operation Mode is not present in Flex Encode, Flex Decode, WP Encode, WP Decode, P200/A200/A300, P100/PF120, P4k/P400

**Endpoint:** `/operationmode`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/operationmode"
```

**Parameters:**

- **Header parameters:**
  - `mode` (string): Device operation mode `encode` `decode`

**Responses:**

- **Status:** default - Current device operation mode is displayed
- **Example data:**
  ```text
  encode
  ```

### operationmodePost

Set base operation modes of your BirdDog device

This command allows you to switch between NDI(R) encode and NDI(R) decode mode on the device - if supported.

Operation Mode is not present in Flex Encode, Flex Decode, WP Encode, WP Decode, P200/A200/A300, P100/PF120, P4k/P400

**Endpoint:** `/operationmode`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: text/plain"\
-d decode http://192.168.2.197:8080/operationmode
```

**Parameters:**

- **Header parameters:**
  - `mode` (string): Device operation mode `encode` `decode`

**Responses:**

- **Status:** default - Once accepted, the API will return confirmation of the new mode set
- **Example data:**
  ```text
  encode
  ```

### videooutputinterfaceGet

Retrieve current video output interface mode of your BirdDog device (sdi/hdmi) or (LowLatency/NormalMode)

This shows you what mode your BirdDog device video output interface is (sdi/hdmi) or (LowLatency/NormalMode)

video output interface Mode is not present in Studio, Mini, Flex Encode, Flex Decode, WP Encode, WP Decode, QUAD, P200/A200/A300, P100/PF120, P4k/P400

**Endpoint:** `/videooutput

interface`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/videooutputinterface"
```

**Parameters:**

- **Header parameters:**
  - `videooutput` (string): Device operation mode `sdi` `hdmi` `LowLatency` `NormalMode`

**Responses:**

- **Status:** default - Current device operation mode is displayed
- **Example data:**
  ```text
  NormalMode
  ```

### videooutputinterfacePost

Set base video output interface mode of your BirdDog device

This command allows you to switch between sdi/hdmi or LowLatency/NormalMode on the device - if supported.

video output interface Mode is not present in Studio, Mini, Flex Encode, Flex Decode, WP Encode, WP Decode, QUAD, P200/A200/A300, P100/PF120, P4k/P400

**Endpoint:** `/videooutputinterface`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: text/plain"\
-d NormalMode http://192.168.2.197:8080/videooutputinterface
```

**Parameters:**

- **Header parameters:**
  - `videooutput` (string): Device video output mode `sdi` `hdmi` `LowLatency` `NormalMode`

**Responses:**

- **Status:** default - Once accepted, the API will return confirmation of the new mode set
- **Example data:**
  ```text
  NormalMode
  ```

## NDIENCODE

### encodeTransportGet

Retrieve current NDI Network Settings

This shows you NDI network settings.

**Endpoint:** `/encodeTransport`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/encodeTransport?Txpm=&Txnetprefix=&Txnetmask=&Txmcttl="
```

**Parameters:**

- **Query parameters:**
  - `Txpm` (string): Multicast, TCP, Multi-TCP, UDP
  - `Txnetprefix` (string): `xxx.xxx.xxx.xxx`
  - `Txnetmask` (string): `xxx.xxx.xxx.xxx`
  - `Txmcttl` (string): `1`

**Responses:**

- **Status:** default - Current NDI network settings are displayed
- **Example data:**
  ```json
  {
    "Txpm": "TCP",
    "Txnetprefix": "xxx.xxx.xxx.xxx",
    "Txnetmask": "xxx.xxx.xxx.xxx",
    "Txmcttl": "1"
  }
  ```

### encodeTransportPost

Set NDI Network Settings

This command allows you to change NDI network settings.

**Endpoint:** `/encodeTransport`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"Txpm":"TCP","Txnetprefix":"xxx.xxx.xxx.xxx","Txnetmask":"xxx.xxx.xxx.xxx","Txmcttl":"1"}' \
http://192.168.2.197:8080/encodeTransport
```

**Parameters:**

- **Query parameters:**
  - `Txpm` (string): Multicast, TCP, Multi-TCP, UDP
  - `Txnetprefix` (string): `xxx.xxx.xxx.xxx`
  - `Txnetmask` (string): `xxx.xxx.xxx.xxx`
  - `Txmcttl` (string): `1`

**Responses:**

- **Status:** default - Once accepted, the API will update NDI network settings.
- **Example data:**
  ```json
  {
    "Txpm": "TCP",
    "Txnetprefix": "xxx.xxx.xxx.xxx",
    "Txnetmask": "xxx.xxx.xxx.xxx",
    "Txmcttl": "1"
  }
  ```

### encodesetupGet

Retrieve Encode Settings

This command will fetch current encode settings.

**Endpoint:** `/encodesetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/encodesetup?ChNum=&VideoFormat=&VideoSampleRate=&ColorBitDepth=&StreamName=&NDIAudio=&ScreenSaverMode=&BandwidthMode=&BandwidthSelect=&LoopTally=&TallyMode=&VideoCSC=&NDIGroup=&NDIGroupName="
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `VideoFormat` (string): VideoFormat
  - `VideoSampleRate` (string): 420, 422, 444
  - `ColorBitDepth` (string): 8Bit,10Bit,12Bit
  - `StreamName` (string): Name of Stream
  - `NDIAudio` (string): NDIAudioMain, NDIAudioAnalog, NDIAudioMute
  - `ScreenSaverMode` (string): BirdDogSS, BlackSS, CaptureSS
  - `BandwidthMode` (string): Manual, NDIManaged
  - `BandwidthSelect` (string): Range (60 to 360)
  - `LoopTally` (string): LoopTallyEn, LoopTallyDis
  - `TallyMode` (string): TallyOn, TallyOff, VideoMode
  - `VideoCSC` (string): NoCSC, RGB, YVU, YUV444, YVU444
  - `NDIGroup` (string): NDIGroupEn, NDIGroupDis
  - `NDIGroupName` (string): BirdDog

**Responses:**

- **Status:** default - Device standard response
- **Example data:**
  ```json
  {
    "ChNum": "1",
    "VideoFormat": "<videoformat>",
    "VideoSampleRate": "420",
    "ColorBitDepth": "8Bit",
    "StreamName": "<name of stream>",
    "NDIAudio": "NDIAudioAnalog",
    "ScreenSaverMode": "CaptureSS",
    "BandwidthMode": "NDIManaged",
    "BandwidthSelect": "120",
    "LoopTally": "LoopTallyDis",
    "TallyMode": "TallyOn",
    "VideoCSC": "RGB",
    "NDIGroup": "NDIGroupDis",
    "NDIGroupName": "<BirdDog>"
  }
  ```

### encodesetupPost

Set Encode Settings

This command sets encode settings.

**Endpoint:** `/encodesetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"ChNum":"1","VideoFormat":"videoformat","VideoSampleRate":"420","ColorBitDepth":"8Bit","StreamName":"name of stream","NDIAudio":"NDIAudioAnalog","ScreenSaverMode":"CaptureSS","BandwidthMode":"NDIManaged","BandwidthSelect":"120","LoopTally":"LoopTallyDis","TallyMode":"TallyOn","VideoCSC":"RGB","NDIGroup":"NDIGroupDis","NDIGroupName":"BirdDog"}' \
http://192.168.2.197:8080/encodesetup
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `VideoFormat` (string): VideoFormat
  - `VideoSampleRate` (string): 420, 422, 444
  - `ColorBitDepth` (string): 8Bit,10Bit,12Bit
  - `StreamName` (string): Name of Stream
  - `NDIAudio` (string): NDIAudioMain, NDIAudioAnalog, NDIAudioMute
  - `ScreenSaverMode` (string): BirdDogSS, BlackSS, CaptureSS
  - `BandwidthMode` (string): Manual, NDIManaged
  - `BandwidthSelect` (string): Range (60 to 360)
  - `LoopTally` (string): LoopTallyEn, LoopTallyDis
  - `TallyMode` (string): TallyOn, TallyOff, VideoMode
  - `VideoCSC` (string): NoCSC, RGB, YVU, YUV444, YVU444
  - `NDIGroup` (string): NDIGroupEn, NDIGroupDis
  - `NDIGroupName` (string): BirdDog

**Responses:**

- **Status:** default - Device standard response
- **Example data:**
  ```json
  {
    "ChNum": "1",
    "VideoFormat": "<videoformat>",
    "VideoSampleRate": "420",
    "ColorBitDepth": "8Bit",
    "StreamName": "<name of stream>",
    "NDIAudio": "NDIAudioAnalog",
    "ScreenSaverMode": "CaptureSS",
    "BandwidthMode": "NDIManaged",
    "BandwidthSelect": "120",
    "LoopTally": "LoopTallyDis",
    "TallyMode": "TallyOn",
    "VideoCSC": "RGB",
    "NDIGroup": "NDIGroupDis",
    "NDIGroupName": "<BirdDog>"
  }
  ```

## NDIDECODE

### captureGet

Capture frame for Encode/Decode

This command will capture screensaver frame for Encode/Decode.

**Endpoint:** `/capture`

**Curl Example:**

```bash
curl -

X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/capture?ChNum=&status="
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `status` (string): Encode, Decode

**Responses:**

- **Status:** default - Capture screensaver frame for Encode/Decode
- **Example data:**
  ```text
  Capture Success..
  ```

### connectToGet

Retrieve connected NDI Source info (sourceName)

This shows you connected NDI Source info (sourceName).

**Endpoint:** `/connectTo`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/connectTo?ChNum=&sourcename=&status="
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `sourcename` (string): None
  - `status` (string): Status can be `Encode` or `Decode`

**Responses:**

- **Status:** default - Current connected NDI Source info (sourceName)
- **Example data:**
  ```json
  {
    "sourceName": "None"
  }
  ```

### connectToPost

Connects to NDI Source

This connects to an NDI Source.

**Endpoint:** `/connectTo`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"sourceName":"None"}' \
http://192.168.2.197:8080/connectTo
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `sourcename` (string): None
  - `status` (string): Status can be `Encode` or `Decode`

**Responses:**

- **Status:** default - Current NDI Source Name is displayed
- **Example data:**
  ```json
  {
    "sourceName": "None"
  }
  ```

### decodeTransportGet

Retrieve NDI network settings

This command will get NDI network settings.

**Endpoint:** `/decodeTransport`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/decodeTransport?rxpm="
```

**Parameters:**

- **Query parameters:**
  - `rxpm` (string): Multicast, TCP, Multi-TCP, UDP

**Responses:**

- **Status:** default - Retrieve NDI network settings
- **Example data:**
  ```json
  {
    "rxpm": "TCP"
  }
  ```

### decodeTransportPost

Set NDI network settings

This command will set NDI network settings.

**Endpoint:** `/decodeTransport`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"rxpm":"TCP"}' \
http://192.168.2.197:8080/decodeTransport
```

**Parameters:**

- **Query parameters:**
  - `rxpm` (string): Multicast, TCP, Multi-TCP, UDP

**Responses:**

- **Status:** default - Set NDI network settings
- **Example data:**
  ```json
  {
    "rxpm": "TCP"
  }
  ```

### decodesetupGet

Retrieve Decode Settings

This command will fetch current decode settings.

**Endpoint:** `/decodesetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/decodesetup?ChNum=&ColorSpace=&TallyMode=&ScreenSaverMode=&NDIAudio="
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `ColorSpace` (string): RGB, YUV
  - `TallyMode` (string): TallyOn, TallyOff, VideoMode
  - `ScreenSaverMode` (string): BirdDogSS, BlackSS, CaptureSS
  - `NDIAudio` (string): NDIAudioEn, NDIAudioDis

**Responses:**

- **Status:** default - Current Decode Settings
- **Example data:**
  ```json
  {
    "ColorSpace": "RGB",
    "NDIAudio": "NDIAudioEn",
    "ScreenSaverMode": "BirdDogSS",
    "TallyMode": "VideoMode",
    "ChNum": "1"
  }
  ```

### decodesetupPost

Set Decode Settings

This command sets decode settings.

**Endpoint:** `/decodesetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"ColorSpace":"RGB","NDIAudio":"NDIAudioEn","ScreenSaverMode":"BirdDogSS","TallyMode":"VideoMode","ChNum":"1"}' \
http://192.168.2.197:8080/decodesetup
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4
  - `ColorSpace` (string): RGB, YUV
  - `TallyMode` (string): TallyOn, TallyOff, VideoMode
  - `ScreenSaverMode` (string): BirdDogSS, BlackSS, CaptureSS
  - `NDIAudio` (string): NDIAudioEn, NDIAudioDis

**Responses:**

- **Status:** default - Current Decode Settings
- **Example data:**
  ```json
  {
    "ColorSpace": "RGB",
    "NDIAudio": "NDIAudioEn",
    "ScreenSaverMode": "BirdDogSS",
    "TallyMode": "VideoMode",
    "ChNum": "1"
  }
  ```

### decodestatusGet

Retrieve connected NDI Source Status

This shows you connected NDI Source Status.

**Endpoint:** `/decodestatus`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/decodestatus?ChNum="
```

**Parameters:**

- **Query parameters:**
  - `ChNum` (string): Channel No. Range 1 to 4

**Responses:**

- **Status:** default - Current connected NDI Source Status
- **Example data:**
  ```json
  {
    "Videoresolution": "0x0",
    "VideoFramerate": "0p",
    "VideoSamplerate": "0:0:0",
    "Audiochannels": "0",
    "AudioSamplerate": "0",
    "AverageBitrate": "0"
  }
  ```

## NDIFINDER

### listGet

Retrieve List of active NDI Sources on the Network

This command will get the list of active NDI Sources on the Network.

**Endpoint:** `/List`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: application/json"\
"http://192.168.2.197:8080/List"
```

**Responses:**

- **Status:** default - List of active NDI Sources on the Network
- **Example data:**
  ```json
  {
    "None": "None"
  }
  ```

### NDIDisServerGet

Retrieve NDI Discovery server info

This command will get NDI Discovery server info.

**Endpoint:** `/NDIDisServer`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/NDIDisServer?NDIDisServ=&NDIDisServIP="
```

**Parameters:**

- **Query parameters:**
  - `NDIDisServ` (string): NDIDisServEn/NDIDisServDis
  - `NDIDisServIP` (string): `xxx.xxx.xxx.xxx`

**Responses:**

- **Status:** default - Retrieve NDI Discovery server info
- **Example data:**
  ```json
  {
    "NDIDisServ": "NDIDisServEn",
    "NDIDisServIP": "xxx.xxx.xxx.xxx"
  }
  ```

### NDIDisServerPost

Set NDI Discovery server info

This command will set NDI Discovery server info.

**Endpoint:** `/NDIDisServer`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"NDIDisServ":"NDIDisServEn","NDIDisServIP":"xxx.xxx.xxx.xxx"}' \
http://192.168.2.197:8080/NDIDisServer
```

**Parameters:**

- **Query parameters:**
  - `NDIDisServ` (string): NDIDisServEn/NDIDisServDis
  - `NDIDisServIP` (string): `xxx.xxx.xxx.xxx`

**Responses:**

- **Status:** default - Set NDI Discovery server info
- **Example data:**
  ```json
  {
    "NDIDisServ": "NDIDisServEn",
    "NDIDisServIP": "xxx.xxx.xxx.xxx"
  }
  ```

### N

DIGrpNameGet
Retrieve GroupName

This command will return Group Name.

**Endpoint:** `/NDIGrpName`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/NDIGrpName"
```

**Responses:**

- **Status:** default - Return Group Name
- **Example data:**
  ```text
  BirdDogxxx
  ```

### NDIGrpNamePost

Set GroupName

This command will set Group Name.

**Endpoint:** `/NDIGrpName`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: text/plain"\
-d "BirdDogxxx" http://192.168.2.197:8080/NdiGrpName
```

**Responses:**

- **Status:** default - Set Group Name
- **Example data:**
  ```text
  BirdDogxxx
  ```

### NDIOffSnSrcGet

Retrieve Off Subnet IP List

This command will return Off Subnet IP List.

**Endpoint:** `/NDIOffSnSrc`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/NDIOffSnSrc"
```

**Responses:**

- **Status:** default - Off Subnet IP List
- **Example data:**
  ```text
  xxx.xxx.xxx.xxx
  ```

### NDIOffSnSrcPost

Set Off Subnet IP

This command will set Off Subnet IP.

**Endpoint:** `/NDIOffSnSrc`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: text/plain"\
-d "xxx.xxx.xxx.xxx" http://192.168.2.197:8080/NdiOffSnSrc
```

**Responses:**

- **Status:** default - Set Off Subnet IP
- **Example data:**
  ```text
  xxx.xxx.xxx.xxx
  ```

### refreshGet

Refresh NDI Source List

This command will immediately refresh NDI Source List.

**Endpoint:** `/refresh`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/refresh"
```

**Responses:**

- **Status:** default - Immediately all the sources on the network will be visible
- **Example data:**
  ```json
  {}
  ```

### refreshPost

Refresh NDI Source List

This command will immediately refresh NDI Source List.

**Endpoint:** `/refresh`

**Curl Example:**

```bash
curl -X POST\
-H "Accept: text"\
"http://192.168.2.197:8080/refresh"
```

**Responses:**

- **Status:** default - Immediately all the sources on the network will be visible
- **Example data:**
  ```json
  {}
  ```

### resetGet

Reset NDI Source List

This command will immediately reset NDI Source List.

**Endpoint:** `/reset`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/reset"
```

**Responses:**

- **Status:** default - Immediately all the sources on the network will be visible
- **Example data:**
  ```json
  {}
  ```

### resetPost

Reset NDI Source List

This command will immediately reset NDI Source List.

**Endpoint:** `/reset`

**Curl Example:**

```bash
curl -X POST\
-H "Accept: text"\
"http://192.168.2.197:8080/reset"
```

**Responses:**

- **Status:** default - Immediately all the sources on the network will be visible
- **Example data:**
  ```json
  {}
  ```

## PTZ

### birddogptzsetupGet

Retrieve PTZ Settings

This command will get PTZ settings.

**Endpoint:** `/birddogptzsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogptzsetup?PanSpeed=&TiltSpeed=&ZoomSpeed="
```

**Parameters:**

- **Query parameters:**
  - `PanSpeed` (string): Range from 0 to 21
  - `TiltSpeed` (string): Range from 0 to 18
  - `ZoomSpeed` (string): Range from 0 to 7

**Responses:**

- **Status:** default - Get PTZ Settings
- **Example data:**
  ```json
  {
    "PanSpeed": "8",
    "TiltSpeed": "8",
    "ZoomSpeed": "4"
  }
  ```

### birddogptzsetupPost

Set PTZ Settings

This command will set PTZ settings.

**Endpoint:** `/birddogptzsetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"PanSpeed":"8","TiltSpeed":"8","ZoomSpeed":"4"}' \
http://192.168.2.197:8080/birddogptzsetup
```

**Parameters:**

- **Query parameters:**
  - `PanSpeed` (string): Range from 0 to 21
  - `TiltSpeed` (string): Range from 0 to 18
  - `ZoomSpeed` (string): Range from 0 to 7

**Responses:**

- **Status:** default - Set PTZ Settings
- **Example data:**
  ```json
  {
    "PanSpeed": "8",
    "TiltSpeed": "8",
    "ZoomSpeed": "4"
  }
  ```

### recallPost

Recall Preset

This command will recall preset.

**Endpoint:** `/recall`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"Preset":"Preset-1"}' \
http://192.168.2.197:8080/recall
```

**Parameters:**

- **Query parameters:**
  - `Preset` (string): Preset-1 (Range from 1 to 9)

**Responses:**

- **Status:** default - Recall Preset
- **Example data:**
  ```json
  {
    "Preset": "Preset-1"
  }
  ```

### savePost

Save Preset

This command will save preset.

**Endpoint:** `/save`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"Preset":"Preset-1"}' \
http://192.168.2.197:8080/save
```

**Parameters:**

- **Query parameters:**
  - `Preset` (string): Preset-1 (Range from 1 to 9)

**Responses:**

- **Status:** default - Save Preset
- **Example data:**
  ```json
  {
    "Preset": "Preset-1"
  }
  ```

## EXPOSURE

### birddogexpsetupGet

Retrieve Exposure Settings

This command will get exposure settings.

**Endpoint:** `/birddogexpsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogexpsetup?AeResponse=&BackLight=&BrightLevel=&ExpCompEn=&ExpCompLvl=&ExpMode=&GainLevel=&GainLimit=&GainPoint=&GainPointPosition=&HighSensitivity=&IrisLevel=&ShutterControlOverwrite=&ShutterMaxSpeed=&ShutterMinSpeed=&ShutterSpeed=&ShutterSpeedOverwrite=&SlowShutterEn=&SlowShutterLimit=&Spotlight="
```

**Parameters:**

- **Query parameters:**
  - `AeResponse` (string): Range (1 to 48)
  - `BackLight` (string): On, Off
  - `BrightLevel` (string): Range (0, 5 to 31)
  - `ExpCompEn` (string): On, Off
  - `ExpCompLvl` (string): Range (0 to 14, -128 to 127)
  - `ExpMode` (string): FULL-AUTO, MANUAL, SHUTTER-PRI, IRIS-PRI, BRIGHT
  - `GainLevel` (string): Range (1 to GainLimit)
  - `GainLimit` (string): Range (4 to 15)
  - `GainPoint` (string): On, Off
  - `GainPointPosition` (string): Range (1 to 13)
  - `HighSensitivity` (string): On, Off
  - `IrisLevel` (string): Range (0, 5 to 17)
  - `ShutterControlOverwrite` (string): On, Off
  - `ShutterMaxSpeed` (string): Range (20 to 33)
  - `ShutterMinSpeed` (string): Range (16 to ShutterMaxSpeed)
  - `ShutterSpeed` (string): Range (0 to 21)
  - `ShutterSpeedOverwrite` (string): Range(30 - 110)
  - `SlowShutterEn` (string): On, Off
  - `SlowShutterLimit` (string): Range (1 to 6)
  - `Spotlight` (string): On, Off

**Responses:**

- **Status:** default - Get Exposure

Settings

- **Example data:**
  ```json
  {
    "AeResponse": "1",
    "BackLight": "Off",
    "BrightLevel": "24",
    "ExpCompEn": "Off",
    "ExpCompLvl": "-123",
    "ExpMode": "FULL-AUTO",
    "GainLevel": "4",
    "GainLimit": "11",
    "GainPoint": "Off",
    "GainPointPosition": "10",
    "HighSensitivity": "Off",
    "IrisLevel": "21",
    "ShutterControlOverwrite": "On",
    "ShutterMaxSpeed": "29",
    "ShutterMinSpeed": "16",
    "ShutterSpeed": "16",
    "ShutterSpeedOverwrite": "30",
    "SlowShutterEn": "Off",
    "SlowShutterLimit": "13",
    "Spotlight": "Off"
  }
  ```

### birddogexpsetupPost

Set Exposure Settings

This command will set exposure settings.

**Endpoint:** `/birddogexpsetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"AeResponse":"1","BackLight":"Off","BrightLevel":"24","ExpCompEn":"Off","ExpCompLvl":"-123","ExpMode":"FULL-AUTO","GainLevel":"4","GainLimit":"11","GainPoint":"Off","GainPointPosition":"10","HighSensitivity":"Off","IrisLevel":"21","ShutterControlOverwrite":"On","ShutterMaxSpeed":"29","ShutterMinSpeed":"16","ShutterSpeed":"16","ShutterSpeedOverwrite":"30","SlowShutterEn":"Off","SlowShutterLimit":"13","Spotlight":"Off"}' \
http://192.168.2.197:8080/birddogexpsetup
```

**Parameters:**

- **Query parameters:**
  - `AeResponse` (string): Range (1 to 48)
  - `BackLight` (string): On, Off
  - `BrightLevel` (string): Range (0, 5 to 31)
  - `ExpCompEn` (string): On, Off
  - `ExpCompLvl` (string): Range (0 to 14, -128 to 127)
  - `ExpMode` (string): FULL-AUTO, MANUAL, SHUTTER-PRI, IRIS-PRI, BRIGHT
  - `GainLevel` (string): Range (1 to GainLimit)
  - `GainLimit` (string): Range (4 to 15)
  - `GainPoint` (string): On, Off
  - `GainPointPosition` (string): Range (1 to 13)
  - `HighSensitivity` (string): On, Off
  - `IrisLevel` (string): Range (0, 5 to 17)
  - `ShutterControlOverwrite` (string): On, Off
  - `ShutterMaxSpeed` (string): Range (20 to 33)
  - `ShutterMinSpeed` (string): Range (16 to ShutterMaxSpeed)
  - `ShutterSpeed` (string): Range (0 to 21)
  - `ShutterSpeedOverwrite` (string): Range(30 - 110)
  - `SlowShutterEn` (string): On, Off
  - `SlowShutterLimit` (string): Range (1 to 6)
  - `Spotlight` (string): On, Off

**Responses:**

- **Status:** default - Set Exposure Settings
- **Example data:**
  ```json
  {
    "AeResponse": "1",
    "BackLight": "Off",
    "BrightLevel": "24",
    "ExpCompEn": "Off",
    "ExpCompLvl": "-123",
    "ExpMode": "FULL-AUTO",
    "GainLevel": "4",
    "GainLimit": "11",
    "GainPoint": "Off",
    "GainPointPosition": "10",
    "HighSensitivity": "Off",
    "IrisLevel": "21",
    "ShutterControlOverwrite": "On",
    "ShutterMaxSpeed": "29",
    "ShutterMinSpeed": "16",
    "ShutterSpeed": "16",
    "ShutterSpeedOverwrite": "30",
    "SlowShutterEn": "Off",
    "SlowShutterLimit": "13",
    "Spotlight": "Off"
  }
  ```

## WHITEBALANCE

### birddogwbsetupGet

Retrieve White Balance Settings

This command will get white balance settings.

**Endpoint:** `/birddogwbsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogwbsetup?BG=&BR=&BlueGain=&ColorTemp=&GB=&GR=&Level=&Matrix=&Offset=&Phase=&RB=&RG=&RedGain=&Select=&Speed=&WbMode="
```

**Parameters:**

- **Query parameters:**
  - `BG` (string): Range (-99 to 99)
  - `BR` (string): Range (-99 to 99)
  - `BlueGain` (string): Range (0 to 255)
  - `ColorTemp` (string): Range (2800 to 6500)
  - `GB` (string): Range (-99 to 99)
  - `GR` (string): Range (-99 to 99)
  - `Level` (string): Range (0 to 14)
  - `Matrix` (string): On, Off
  - `Offset` (string): Range (0 to 14)
  - `Phase` (string): Range (0 to 14)
  - `RB` (string): Range (-99 to 99)
  - `RG` (string): Range (-99 to 99)
  - `RedGain` (string): Range (0 to 255)
  - `Select` (string): FL LIGHT, STD, OFF, HIGH SAT
  - `Speed` (string): Range (1 to 5)
  - `WbMode` (string): AUTO, INDOOR, OUTDOOR, OUTDOOR-AUTO, ONEPUSH, ATW, MANUAL, SVL-AUTO, SVL, SVL-OUTDOOR-AUTO

**Responses:**

- **Status:** default - Get White Balance Settings
- **Example data:**
  ```json
  {
    "BG": "0",
    "BR": "0",
    "BlueGain": "174",
    "ColorTemp": "2800",
    "GB": "0",
    "GR": "0",
    "Level": "4",
    "Matrix": "Off",
    "Offset": "7",
    "Phase": "7",
    "RB": "0",
    "RG": "0",
    "RedGain": "179",
    "Select": "OFF",
    "Speed": "3",
    "WbMode": "AUTO"
  }
  ```

### birddogwbsetupPost

Set White Balance Settings

This command will set white balance settings.

**Endpoint:** `/birddogwbsetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"BG":"0","BR":"0","BlueGain":"174","ColorTemp":"2800","GB":"0","GR":"0","Level":"4","Matrix":"Off","Offset":"7","Phase":"7","RB":"0","RG":"0","RedGain":"179","Select":"OFF","Speed":"3","WbMode":"AUTO"}' \
http://192.168.2.197:8080/birddogwbsetup
```

**Parameters:**

- **Query parameters:**
  - `BG` (string): Range (-99 to 99)
  - `BR` (string): Range (-99 to 99)
  - `BlueGain` (string): Range (0 to 255)
  - `ColorTemp` (string): Range (2800 to 6500)
  - `GB` (string): Range (-99 to 99)
  - `GR` (string): Range (-99 to 99)
  - `Level` (string): Range (0 to 14)
  - `Matrix` (string): On, Off
  - `Offset` (string): Range (0 to 14)
  - `Phase` (string): Range (0 to 14)
  - `RB` (string): Range (-99 to 99)
  - `RG` (string): Range (-99 to 99)
  - `RedGain` (string): Range (0 to 255)
  - `Select` (string): FL LIGHT, STD, OFF, HIGH SAT
  - `Speed` (string): Range (1 to 5)
  - `WbMode` (string): AUTO, INDOOR, OUTDOOR, OUTDOOR-AUTO, ONEPUSH, ATW, MANUAL, SVL-AUTO, SVL, SVL-OUTDOOR-AUTO

**Responses:**

- **Status:** default - Set White Balance Settings
- **Example data:**

  ```json
  {
    "BG": "0",
    "BR": "0",
    "BlueGain": "174",
    "ColorTemp": "2800",
    "GB": "0",
    "GR": "0",
    "Level": "4",
    "Matrix": "Off",

    "Offset": "7",
    "Phase": "7",
    "RB": "0",
    "RG": "0",
    "RedGain": "179",
    "Select": "OFF",
    "Speed": "3",
    "WbMode": "AUTO"
  }
  ```

## PICTURESETTINGS

### birddogpicsetupGet

Retrieve Picture Settings

This command will get picture settings.

**Endpoint:** `/birddogpicsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogpicsetup?BackLightCom=&ChromeSuppress=&Color=&Contrast=&Effect=&Flip=&Gamma=&HighlightComp=&HighlightCompMask=&Hue=&IRCutFilter=&Mirror=&NoiseReduction=&Sharpness=&Stabilizer=&TWODNR=&ThreeDNR=&WideDynamicRange=&LowLatency=&NDFilter"
```

**Parameters:**

- **Query parameters:**
  - `BackLightCom` (string): On, Off
  - `ChromeSuppress` (string): OFF, LOW, MEDIUM, HIGH
  - `Color` (string): Range (0 to 15)
  - `Contrast` (string): Range (0 to 15)
  - `Effect` (string): BW, Off
  - `Flip` (string): On, Off
  - `Gamma` (string): Range (0 to 4)
  - `HighlightComp` (string): OFF, LOW, MEDIUM, HIGH
  - `HighlightCompMask` (string): Range (0 to 15)
  - `Hue` (string): Range (0 to 15)
  - `IRCutFilter` (string): Auto, On, Off
  - `Mirror` (string): On, Off
  - `NoiseReduction` (string): Range (Off, 1 to 5)
  - `Sharpness` (string): Range (-128 to 127)
  - `Stabilizer` (string): On, Off
  - `TWODNR` (string): Range (Off, 1 to 5)
  - `ThreeDNR` (string): Range (Off, 1 to 5)
  - `WideDynamicRange` (string): Range (Off, 1 to 6)
  - `LowLatency` (string): On, Off
  - `NDFilter` (string): Range (0 to 3)

**Responses:**

- **Status:** default - Get Picture Settings
- **Example data:**
  ```json
  {
    "BackLightCom": "On",
    "ChromeSuppress": "OFF",
    "Color": "8",
    "Contrast": "1",
    "Effect": "BW",
    "Flip": "Off",
    "Gamma": "1",
    "HighlightComp": "On",
    "HighlightCompMask": "3",
    "Hue": "7",
    "IRCutFilter": "Off",
    "Mirror": "Off",
    "NoiseReduction": "Off",
    "Sharpness": "122",
    "Stabilizer": "Off",
    "TWODNR": "2",
    "ThreeDNR": "2",
    "WideDynamicRange": "Off",
    "LowLatency": "Off",
    "NDFilter": "2"
  }
  ```

### birddogpicsetupPost

Set Picture Settings

This command will set picture settings.

**Endpoint:** `/birddogpicsetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"BackLightCom":"On","ChromeSuppress":"Off","Color":"8","Contrast":"1","Effect":"BW","Flip":"Off","Gamma":"1","HighlightComp":"On","HighlightCompMask":"3","Hue":"7","IRCutFilter":"Off","Mirror":"Off","NoiseReduction":"Off","Sharpness":"122","Stabilizer":"Off","TWODNR":"2","ThreeDNR":"2","WideDynamicRange":"Off","LowLatency":"Off","NDFilter":"2"}' \
http://192.168.2.197:8080/birddogpicsetup
```

**Parameters:**

- **Query parameters:**
  - `BackLightCom` (string): On, Off
  - `ChromeSuppress` (string): OFF, LOW, MEDIUM, HIGH
  - `Color` (string): Range (0 to 15)
  - `Contrast` (string): Range (0 to 15)
  - `Effect` (string): BW, Off
  - `Flip` (string): On, Off
  - `Gamma` (string): Range (0 to 4)
  - `HighlightComp` (string): OFF, LOW, MEDIUM, HIGH
  - `HighlightCompMask` (string): Range (0 to 15)
  - `Hue` (string): Range (0 to 15)
  - `IRCutFilter` (string): Auto, On, Off
  - `Mirror` (string): On, Off
  - `NoiseReduction` (string): Range (Off, 1 to 5)
  - `Sharpness` (string): Range (-128 to 127)
  - `Stabilizer` (string): On, Off
  - `TWODNR` (string): Range (Off, 1 to 5)
  - `ThreeDNR` (string): Range (Off, 1 to 5)
  - `WideDynamicRange` (string): Range (Off, 1 to 6)
  - `LowLatency` (string): On, Off
  - `NDFilter` (string): Range (0 to 3)

**Responses:**

- **Status:** default - Set Picture Settings
- **Example data:**
  ```json
  {
    "BackLightCom": "On",
    "ChromeSuppress": "Off",
    "Color": "8",
    "Contrast": "1",
    "Effect": "BW",
    "Flip": "Off",
    "Gamma": "1",
    "HighlightComp": "On",
    "HighlightCompMask": "3",
    "Hue": "7",
    "IRCutFilter": "Off",
    "Mirror": "Off",
    "NoiseReduction": "Off",
    "Sharpness": "122",
    "Stabilizer": "Off",
    "TWODNR": "2",
    "ThreeDNR": "2",
    "WideDynamicRange": "Off",
    "LowLatency": "Off",
    "NDFilter": "2"
  }
  ```

## COLOURMATRIX

### birddogcmsetupGet

Retrieve Colour Matrix Settings

**Endpoint:** `/birddogcmsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogcmsetup?BlueGain=&BlueHue=&ColourGain=&CyanGain=&CyanHue=&GreenGain=&GreenHue=&HuePhase=&MagGain=&MagHue=&RedGain=&RedHue=&YellowGain=&YellowHue="
```

**Parameters:**

- **Query parameters:**
  - `BlueGain` (string): Range (0 to 64)
  - `BlueHue` (string): Range (0 to 64)
  - `ColourGain` (string): Range (0 to 255)
  - `CyanGain` (string): Range (0 to 64)
  - `CyanHue` (string): Range (0 to 64)
  - `GreenGain` (string): Range (0 to 64)
  - `GreenHue` (string): Range (0 to 64)
  - `HuePhase` (string): Range (0 to 255)
  - `MagGain` (string): Range (0 to 64)
  - `MagHue` (string): Range (0 to 64)
  - `RedGain` (string): Range (0 to 64)
  - `RedHue` (string): Range (0 to 64)
  - `YellowGain` (string): Range (0 to 64)
  - `YellowHue` (string): Range (0 to 64)

**Responses:**

- **Status:** default - Get Colour Matrix Settings
- **Example data:**
  ```json
  {
    "BlueGain": "32",
    "BlueHue": "32",
    "ColourGain": "128",
    "CyanGain": "32",
    "CyanHue": "32",
    "GreenGain": "32",
    "GreenHue": "32",
    "HuePhase": "128",
    "MagGain": "32",
    "MagHue": "32",
    "RedGain": "32",
    "RedHue": "32",
    "YellowGain": "32",
    "YellowHue": "32"
  }
  ```

### birddogcmsetupPost

Set Colour Matrix Settings

**Endpoint:** `/birddogcmsetup`

**Curl Example:**

````bash
curl -X POST\
-H "Accept: application/json"\
-d '{"BlueGain":"32","BlueHue":"32","ColourGain":"128","CyanGain":"32","CyanHue":"32","GreenGain":"32","GreenHue":"32","HuePhase":"128","MagGain":"32","MagHue":"32","RedGain":"32","RedHue":"32","YellowGain":"32","YellowHue":"32"}' \
http://192.168.2.197:8080/birddogcmsetup
``

`

**Parameters:**
- **Query parameters:**
    - `BlueGain` (string): Range (0 to 64)
    - `BlueHue` (string): Range (0 to 64)
    - `ColourGain` (string): Range (0 to 255)
    - `CyanGain` (string): Range (0 to 64)
    - `CyanHue` (string): Range (0 to 64)
    - `GreenGain` (string): Range (0 to 64)
    - `GreenHue` (string): Range (0 to 64)
    - `HuePhase` (string): Range (0 to 255)
    - `MagGain` (string): Range (0 to 64)
    - `MagHue` (string): Range (0 to 64)
    - `RedGain` (string): Range (0 to 64)
    - `RedHue` (string): Range (0 to 64)
    - `YellowGain` (string): Range (0 to 64)
    - `YellowHue` (string): Range (0 to 64)

**Responses:**
- **Status:** default - Set Colour Matrix Settings
- **Example data:**
    ```json
    {
      "BlueGain": "32",
      "BlueHue": "32",
      "ColourGain": "128",
      "CyanGain": "32",
      "CyanHue": "32",
      "GreenGain": "32",
      "GreenHue": "32",
      "HuePhase": "128",
      "MagGain": "32",
      "MagHue": "32",
      "RedGain": "32",
      "RedHue": "32",
      "YellowGain": "32",
      "YellowHue": "32"
    }
    ```

## ADVANCEDSETTINGS

### birddogadvancesetupGet
Retrieve Advanced Settings

**Endpoint:** `/birddogadvancesetup`

**Curl Example:**
```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogadvancesetup?Brightness=&BrightnessComp=&CompLevel=&GammaOffset=&HighResolution=&VideoEnhancement="
````

**Parameters:**

- **Query parameters:**
  - `Brightness` (string): Range (0 to 6)
  - `BrightnessComp` (string): VERY DARK, DARK, STANDARD, BRIGHT
  - `CompLevel` (string): LOW, MID, HIGH
  - `GammaOffset` (string): Range (16 to 64)
  - `HighResolution` (string): On, Off
  - `VideoEnhancement` (string): On, Off

**Responses:**

- **Status:** default - Get Advanced Settings
- **Example data:**
  ```json
  {
    "Brightness": "2",
    "BrightnessComp": "STANDARD",
    "CompLevel": "LOW",
    "GammaOffset": "16",
    "HighResolution": "Off",
    "VideoEnhancement": "Off"
  }
  ```

### birddogadvancesetupPost

Set Advanced Settings

**Endpoint:** `/birddogadvancesetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"Brightness":"2","BrightnessComp":"STANDARD","CompLevel":"LOW","GammaOffset":"16","HighResolution":"Off","VideoEnhancement":"Off"}' \
http://192.168.2.197:8080/birddogadvancesetup
```

**Parameters:**

- **Query parameters:**
  - `Brightness` (string): Range (0 to 6)
  - `BrightnessComp` (string): VERY DARK, DARK, STANDARD, BRIGHT
  - `CompLevel` (string): LOW, MID, HIGH
  - `GammaOffset` (string): Range (16 to 64)
  - `HighResolution` (string): On, Off
  - `VideoEnhancement` (string): On, Off

**Responses:**

- **Status:** default - Set Advanced Settings
- **Example data:**
  ```json
  {
    "Brightness": "2",
    "BrightnessComp": "STANDARD",
    "CompLevel": "LOW",
    "GammaOffset": "16",
    "HighResolution": "Off",
    "VideoEnhancement": "Off"
  }
  ```

## EXTERNALSETTINGS

### birddogexternalsetupGet

Retrieve External Settings

This command will get External settings.

**Endpoint:** `/birddogexternalsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogexternalsetup?Aux=&RainWiper=&V12vOut="
```

**Parameters:**

- **Query parameters:**
  - `Aux` (string): On, Off
  - `RainWiper` (string): On, Off
  - `V12vOut` (string): On, Off

**Responses:**

- **Status:** default - Get External Settings
- **Example data:**
  ```json
  {
    "Aux": "Off",
    "RainWiper": "Off",
    "V12vOut": "Off"
  }
  ```

### birddogexternalsetupPost

Set External Settings

This command will Set External settings.

**Endpoint:** `/birddogexternalsetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"Aux":"Off","RainWiper":"Off","V12vOut":"Off"}' \
http://192.168.2.197:8080/birddogexternalsetup
```

**Parameters:**

- **Query parameters:**
  - `Aux` (string): On, Off
  - `RainWiper` (string): On, Off
  - `V12vOut` (string): On, Off

**Responses:**

- **Status:** default - Set External Settings
- **Example data:**
  ```json
  {
    "Aux": "Off",
    "RainWiper": "Off",
    "V12vOut": "Off"
  }
  ```

## DETAIL

### birddogdetsetupGet

Retrieve detail Settings

**Endpoint:** `/birddogdetsetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogdetsetup?Bandwidth=&BwBalance=&Crispening=&Detail=&HighLightDetail=&HvBalance=&Limit=&SuperLow="
```

**Parameters:**

- **Query parameters:**
  - `Bandwidth` (string): DEFAULT, LOW, MIDDLE, HIGH, WIDE
  - `BwBalance` (string): TYPE1, TYPE2, TYPE3, TYPE4, TYPE5
  - `Crispening` (string): Range (0 to 7)
  - `Detail` (string): On, Off
  - `HighLightDetail` (string): Range (0 to 4)
  - `HvBalance` (string): Range (-2 to 2)
  - `Limit` (string): Range (0 to 7)
  - `SuperLow` (string): Range (0 to 7)

**Responses:**

- **Status:** default - Get detail Settings
- **Example data:**
  ```json
  {
    "Bandwidth": "DEFAULT",
    "BwBalance": "TYPE1",
    "Crispening": "0",
    "Detail": "On",
    "HighLightDetail": "0",
    "HvBalance": "-2",
    "Level": "3",
    "Limit": "3",
    "SuperLow": "0"
  }
  ```

### birddogdetsetupPost

Set detail Settings

**Endpoint:** `/birddogdetsetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"Bandwidth":"DEFAULT","BwBalance":"TYPE1","Crispening":"0","Detail":"On","HighLightDetail":"0","HvBalance":"-2","Level":"3","Limit":"3","SuperLow":"0"}' \
http://192.168.2.197:8080/birddogdetsetup
```

**Parameters:**

- **Query parameters:**
  - `Bandwidth` (string): DEFAULT, LOW, MIDDLE, HIGH, WIDE
  - `BwBalance` (string): TYPE1, TYPE2, TYPE3, TYPE4, TYPE5
  - `Crispening` (string): Range (0 to 7)
  - `Detail` (string): On, Off
  - `HighLightDetail` (string): Range (0 to 4)
  - `HvBalance` (string): Range (-2 to 2)
  - `Limit` (string): Range (0 to 7)
  - `SuperLow` (string): Range (0 to 7)

**Responses:**

- **Status:** default - Set detail Settings
- **Example data:**
  ```json
  {
    "Bandwidth": "DEFAULT",
    "BwBalance": "TYPE1",
    "Crispening": "0",
    "Detail": "On",
    "HighLightDetail": "0",
    "HvBalance": "-2",
    "Level": "3",
    "Limit": "3",
    "SuperLow": "0"
  }
  ```

## GAM

MA

### birddoggammasetupGet

Retrieve Gamma Settings

This command will get Gamma settings.

**Endpoint:** `/birddoggammasetup`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddoggammasetup?BlackGammaLevel=&BlackLevel=&BlackLevelRange=&Effect=&Level=&Offset=&Pattern=&PatternFine=&Settings=&VisibilityEnhancer="
```

**Parameters:**

- **Query parameters:**
  - `BlackGammaLevel` (string): Range (0 to 14)
  - `BlackLevel` (string): Range (0 to 96)
  - `BlackLevelRange` (string): LOW, MID, HIGH
  - `Effect` (string): Range (-3 to 3)
  - `Level` (string): Range (0 to 14)
  - `Offset` (string): Range (-64 to 64)
  - `Pattern` (string): Range (0 to 512)
  - `PatternFine` (string): Range (0 to9)
  - `Settings` (string): PATTERN, STANDARD, STRAIGHT
  - `VisibilityEnhancer` (string): On, Off

**Responses:**

- **Status:** default - Get Gamma Settings
- **Example data:**
  ```json
  {
    "BlackGammaLevel": "7",
    "BlackLevel": "0",
    "BlackLevelRange": "LOW",
    "Effect": "0",
    "Level": "7",
    "Offset": "0",
    "Pattern": "51",
    "PatternFine": "2",
    "Settings": "STANDARD",
    "VisibilityEnhancer": "Off"
  }
  ```

### birddoggammasetupPost

Set Gamma Settings

This command will set Gamma settings.

**Endpoint:** `/birddoggammasetup`

**Curl Example:**

```bash
curl -X POST\
-H "Content-Type: application/json"\
-d '{"BlackGammaLevel":"7","BlackLevel":"0","BlackLevelRange":"LOW","Effect":"0","Level":"7","Offset":"0","Pattern":"51","PatternFine":"2","Settings":"STANDARD","VisibilityEnhancer":"Off"}' \
http://192.168.2.197:8080/birddoggammasetup
```

**Parameters:**

- **Query parameters:**
  - `BlackGammaLevel` (string): Range (0 to 14)
  - `BlackLevel` (string): Range (0 to 96)
  - `BlackLevelRange` (string): LOW, MID, HIGH
  - `Effect` (string): Range (-3 to 3)
  - `Level` (string): Range (0 to 14)
  - `Offset` (string): Range (-64 to 64)
  - `Pattern` (string): Range (0 to 512)
  - `PatternFine` (string): Range (0 to9)
  - `Settings` (string): PATTERN, STANDARD, STRAIGHT
  - `VisibilityEnhancer` (string): On, Off

**Responses:**

- **Status:** default - Set Gamma Settings
- **Example data:**
  ```json
  {
    "BlackGammaLevel": "7",
    "BlackLevel": "0",
    "BlackLevelRange": "LOW",
    "Effect": "0",
    "Level": "7",
    "Offset": "0",
    "Pattern": "51",
    "PatternFine": "2",
    "Settings": "STANDARD",
    "VisibilityEnhancer": "Off"
  }
  ```

## SILICON2

### birddogsil2codecGet

Retrieve Silicon2 Codec Settings

This command will get Silicon2 Codec settings.

**Endpoint:** `/birddogsil2codec`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogsil2codec?Protocol=&SelectedMode=&QuantFactorI=&QuantFactorP=&GOPSize=&Bitrate="
```

**Parameters:**

- **Query parameters:**
  - `Protocol` (string): RTSP, SRT, HX, RTMP, DISABLE
  - `SelectedMode` (string): Custom, low, med, high, ultra
  - `QuantFactorI` (string): Range (18 to 47)
  - `QuantFactorP` (string): Range (18 to 47)
  - `GOPSize` (string): Range (1 to 59)
  - `Bitrate` (string): Range (1 to 80)

**Responses:**

- **Status:** default - Get Silicon2 Codec Settings
- **Example data:**
  ```json
  {
    "DISABLE": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "Custom",
      "high": {
        "Bitrate": "10",
        "GOPSize": "30",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "low": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "med": {
        "Bitrate": "8",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ultra": {
        "Bitrate": "50",
        "GOPSize": "1",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      }
    },
    "HX": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "high",
      "high": {
        "Bitrate": "18",
        "GOPSize": "30",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "low": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "med": {
        "Bitrate": "8",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ultra": {
        "Bitrate": "50",
        "GOPSize": "1",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      }
    },
    "RTMP": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "59",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "Custom",
      "high": {
        "Bitrate": "10",
        "GOPSize": "30",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "low": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "med": {
        "Bitrate": "8",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ultra": {
        "Bitrate": "50",
        "GOPSize": "1",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      }
    },
    "RTSP": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "59",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "Custom",
      "high": {
        "Bitrate": "10",
        "GOPSize": "30",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "low": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "med": {
        "Bitrate": "8",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ultra": {
        "Bitrate": "50",
        "GOPSize": "1",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      }
    },
    "SRT": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "59",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "Custom",
      "high
  ```

": {
"Bitrate": "10",
"GOPSize": "30",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"low": {
"Bitrate": "3",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"med": {
"Bitrate": "8",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ultra": {
"Bitrate": "50",
"GOPSize": "1",
"QuantFactorI": "30",
"QuantFactorP": "30"
}
}
}
```

### birddogsil2codecPost

Set Silicon2 Codec Settings

This command will set Silicon2 Codec settings.

**Endpoint:** `/birddogsil2codec`

**Curl Example:**

```bash
curl --location 'http://192.168.2.197:8080/sil2codec' \
--header 'Content-Type: application/json' \
--data '{
  "DISABLE": {
    "BitrateControl": "cbr",
    "Custom": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ModeSel": "Custom",
    "high": {
      "Bitrate": "10",
      "GOPSize": "30",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "low": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "med": {
      "Bitrate": "8",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ultra": {
      "Bitrate": "50",
      "GOPSize": "1",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    }
  },
  "HX": {
    "BitrateControl": "cbr",
    "Custom": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ModeSel": "high",
    "high": {
      "Bitrate": "18",
      "GOPSize": "30",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "low": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "med": {
      "Bitrate": "8",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ultra": {
      "Bitrate": "50",
      "GOPSize": "1",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    }
  },
  "RTMP": {
    "BitrateControl": "cbr",
    "Custom": {
      "Bitrate": "3",
      "GOPSize": "59",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ModeSel": "Custom",
    "high": {
      "Bitrate": "10",
      "GOPSize": "30",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "low": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "med": {
      "Bitrate": "8",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ultra": {
      "Bitrate": "50",
      "GOPSize": "1",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    }
  },
  "RTSP": {
    "BitrateControl": "cbr",
    "Custom": {
      "Bitrate": "3",
      "GOPSize": "59",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ModeSel": "Custom",
    "high": {
      "Bitrate": "10",
      "GOPSize": "30",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "low": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "med": {
      "Bitrate": "8",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ultra": {
      "Bitrate": "50",
      "GOPSize": "1",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    }
  },
  "SRT": {
    "BitrateControl": "cbr",
    "Custom": {
      "Bitrate": "3",
      "GOPSize": "59",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ModeSel": "Custom",
    "high": {
      "Bitrate": "10",
      "GOPSize": "30",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "low": {
      "Bitrate": "3",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "med": {
      "Bitrate": "8",
      "GOPSize": "60",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    },
    "ultra": {
      "Bitrate": "50",
      "GOPSize": "1",
      "QuantFactorI": "30",
      "QuantFactorP": "30"
    }
  }
}'
```

**Parameters:**

- **Query parameters:**
  - `Protocol` (string): SRT, RTSP, RTMP, HX, DISABLE
  - `BitrateControl` (string): vbr, cbr
  - `ModeSel` (string): Custom, low, med, high, ultra
  - `QuantFactorI` (string): Range (18 to 47)
  - `QuantFactorP` (string): Range (18 to 47)
  - `GOPSize` (string): Range (1 to 59)
  - `Bitrate` (string): Range (1 to 80)

**Responses:**

- **Status:** default - Set Silicon2 Codec Settings
- **Example data:**
  ```json
  {
    "DISABLE": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "Custom",
      "high": {
        "Bitrate": "10",
        "GOPSize": "30",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "low": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "med": {
        "Bitrate": "8",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ultra": {
        "Bitrate": "50",
        "GOPSize": "1",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      }
    },
    "HX": {
      "BitrateControl": "cbr",
      "Custom": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ModeSel": "high",
      "high": {
        "Bitrate": "18",
        "GOPSize": "30",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "low": {
        "Bitrate": "3",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "med": {
        "Bitrate": "8",
        "GOPSize": "60",
        "QuantFactorI": "30",
        "QuantFactorP": "30"
      },
      "ultra": {
        "Bitrate": "50",
        "GOPSize
  ```

": "1",
"QuantFactorI": "30",
"QuantFactorP": "30"
}
},
"RTMP": {
"BitrateControl": "cbr",
"Custom": {
"Bitrate": "3",
"GOPSize": "59",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ModeSel": "Custom",
"high": {
"Bitrate": "10",
"GOPSize": "30",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"low": {
"Bitrate": "3",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"med": {
"Bitrate": "8",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ultra": {
"Bitrate": "50",
"GOPSize": "1",
"QuantFactorI": "30",
"QuantFactorP": "30"
}
},
"RTSP": {
"BitrateControl": "cbr",
"Custom": {
"Bitrate": "3",
"GOPSize": "59",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ModeSel": "Custom",
"high": {
"Bitrate": "10",
"GOPSize": "30",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"low": {
"Bitrate": "3",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"med": {
"Bitrate": "8",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ultra": {
"Bitrate": "50",
"GOPSize": "1",
"QuantFactorI": "30",
"QuantFactorP": "30"
}
},
"SRT": {
"BitrateControl": "cbr",
"Custom": {
"Bitrate": "3",
"GOPSize": "59",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ModeSel": "Custom",
"high": {
"Bitrate": "10",
"GOPSize": "30",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"low": {
"Bitrate": "3",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"med": {
"Bitrate": "8",
"GOPSize": "60",
"QuantFactorI": "30",
"QuantFactorP": "30"
},
"ultra": {
"Bitrate": "50",
"GOPSize": "1",
"QuantFactorI": "30",
"QuantFactorP": "30"
}
}
}
```

### birddogsil2encGet

Retrieve Silicon2 Encode Settings

This command will get Silicon2 Encode settings.

**Endpoint:** `/birddogsil2enc`

**Curl Example:**

```bash
curl -X GET\
-H "Accept: text"\
"http://192.168.2.197:8080/birddogsil2enc"
```

**Parameters:**

- **Query parameters:**
  - `Streaming Protocol` (string): RTSP, SRT, HX, RTMP, DISABLE
  - `Port` (string): RTSP Port
  - `Stream Name` (string): RTSP Stream Name
  - `AuthEnable` (string): RTSP Encryption
  - `UserName` (string): Minimum 4 Characters
  - `Password` (string): Minimum 4 Characters
  - `IPAddress` (string): SRT IP Address
  - `Port` (string): Port For SRT
  - `mode` (string): caller, listener, rendezvous
  - `latency` (string): Range (80 to 8000)
  - `Encryption` (string): true / false
  - `passphrase` (string): Text Length (10 to 79 Characters)
  - `pbkeylen` (string): 16, 24, 32
  - `streamid` (string): up to 512
  - `Server Selection` (string): RTMP - local, remote
  - `AuthEnable` (string): true / false
  - `StreamKeyLocal` (string): Text Length (10 to 79 Characters)
  - `StreamKeyRemote` (string): Text Length (10 to 79 Characters)
  - `Server` (string): Remote Server URL
  - `UserName` (string): Minimum 4 Characters
  - `Password` (string): Minimum 4 Characters
  - `HaiVisionPlayerSupport` (string): true / false

**Responses:**

- **Status:** default - Get Silicon2 Encode Settings
- **Example data:**
  ```json
  {
    "HaiVisionPlayerSupport": "true",
    "RTMP": {
      "AuthEnable": "0",
      "ConnectionURL": "rtmp://192.168.2.197:1935/bdlive/birddogkey",
      "Password": "testpwd",
      "Server": "TestUrl",
      "ServerSelection": "local",
      "StreamKeyLocal": "birddogkey",
      "StreamKeyRemote": "remotekey",
      "UserName": "testuser"
    },
    "RTSP": {
      "AuthEnable": "0",
      "ConnectionURL": "rtsp://192.168.2.197:3489/birddog-stream",
      "Password": "",
      "Port": "3489",
      "StreamName": "birddog-stream",
      "UserName": ""
    },
    "SRT": {
      "ConnectionURL": "srt://192.168.2.197:7900?mode=listener&latency=120",
      "Encryption": "false",
      "IPAddress": "",
      "Port": "7900",
      "latency": "120",
      "mode": "caller",
      "passphrase": "",
      "pbkeylen": "32",
      "streamid": "21"
    },
    "StreamingProtocol": "SRT"
  }
  ```

### birddogsil2encPost

Set Silicon2 Encode Settings

This command will set Silicon2 Encode settings.

**Endpoint:** `/birddogsil2enc`

**Curl Example:**

```bash
curl --location 'http://192.168.2.197:8080/sil2enc' \
--header 'Content-Type: application/json' \
--data '{
  "HaiVisionPlayerSupport": "true",
  "RTMP": {
    "AuthEnable": "0",
    "ConnectionURL": "rtmp://192.168.2.197:1935/bdlive/birddogkey",
    "Password": "testpwd",
    "Server": "TestUrl",
    "ServerSelection": "local",
    "StreamKeyLocal": "birddogkey",
    "StreamKeyRemote": "remotekey",
    "UserName": "testuser"
  },
  "RTSP": {
    "AuthEnable": "0",
    "ConnectionURL": "rtsp://192.168.2.197:3489/birddog-stream",
    "Password": "",
    "Port": "3489",
    "StreamName": "birddog-stream",
    "UserName": ""
  },
  "SRT": {
    "ConnectionURL": "srt://192.168.2.197:7900?mode=listener&latency=120",
    "Encryption": "false",
    "IPAddress": "",
    "Port": "7900",
    "latency": "120",
    "mode": "caller",
    "passphrase": "",
    "pbkeylen": "32",
    "streamid": "21"
  },
  "StreamingProtocol": "SRT"
}'
```

**Parameters:**

- **Query parameters:**
  - `Streaming Protocol` (string): RTSP, SRT, HX, RTMP, DISABLE
  - `Port` (string): RTSP Port
  - `Stream Name` (string): RTSP Stream Name
  - `AuthEnable` (string): RTSP Encryption
  - `UserName` (string): Minimum 4 Characters
  - `Password` (string): Minimum 4 Characters
  - `IPAddress` (string): SRT IP Address
  - `Port` (string): Port For SRT
  - `mode` (string): caller, listener, rendezvous
  - `latency` (string): Range (80 to 8000)
  - `Encryption` (string): true / false
  - `passphrase` (string): Text Length (10 to 79 Characters)
  - `pbkeylen` (string): 16, 24,

32 - `streamid` (string): up to 512 - `Server Selection` (string): RTMP - local, remote - `AuthEnable` (string): true / false - `StreamKeyLocal` (string): Text Length (10 to 79 Characters) - `StreamKeyRemote` (string): Text Length (10 to 79 Characters) - `Server` (string): Remote Server URL - `UserName` (string): Minimum 4 Characters - `Password` (string): Minimum 4 Characters - `HaiVisionPlayerSupport` (string): true / false

**Responses:**

- **Status:** default - Set Silicon2 Encode Settings
- **Example data:**
  ```json
  {
    "HaiVisionPlayerSupport": "true",
    "RTMP": {
      "AuthEnable": "0",
      "ConnectionURL": "rtmp://192.168.2.197:1935/bdlive/birddogkey",
      "Password": "testpwd",
      "Server": "TestUrl",
      "ServerSelection": "local",
      "StreamKeyLocal": "birddogkey",
      "StreamKeyRemote": "remotekey",
      "UserName": "testuser"
    },
    "RTSP": {
      "AuthEnable": "0",
      "ConnectionURL": "rtsp://192.168.2.197:3489/birddog-stream",
      "Password": "",
      "Port": "3489",
      "StreamName": "birddog-stream",
      "UserName": ""
    },
    "SRT": {
      "ConnectionURL": "srt://192.168.2.197:7900?mode=listener&latency=120",
      "Encryption": "false",
      "IPAddress": "",
      "Port": "7900",
      "latency": "120",
      "mode": "caller",
      "passphrase": "",
      "pbkeylen": "32",
      "streamid": "21"
    },
    "StreamingProtocol": "SRT"
  }
  ```
