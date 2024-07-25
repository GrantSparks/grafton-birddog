# Rust API client for openapi

Unofficial idiomatic rust bindings for the BirdDog RESTful API 2.0 generated using OpenAPI Generator.

## Overview

This is a project by the [Grafton Machine Shed](https://www.grafton.ai) and while we use it ourselves, we're only testing functionality as we need it in our own projects.

The API client was generated by the [OpenAPI Generator](https://openapi-generator.tech).  

## Installation

Add the following to `Cargo.toml` under `[dependencies]`:

```
grafton-birddog = "*"
```

## Documentation for API Endpoints

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**about_get**](docs/DefaultApi.md#about_get) | **GET** /about | Retrieve basic information from your BirdDog device
*DefaultApi* | [**analogaudiosetup_get**](docs/DefaultApi.md#analogaudiosetup_get) | **GET** /analogaudiosetup | Retrieve Audio Settings
*DefaultApi* | [**analogaudiosetup_post**](docs/DefaultApi.md#analogaudiosetup_post) | **POST** /analogaudiosetup | Set Audio Settings
*DefaultApi* | [**birddogadvancesetup_get**](docs/DefaultApi.md#birddogadvancesetup_get) | **GET** /birddogadvancesetup | Retrieve Advanced Settings
*DefaultApi* | [**birddogadvancesetup_post**](docs/DefaultApi.md#birddogadvancesetup_post) | **POST** /birddogadvancesetup | Set Advanced Settings
*DefaultApi* | [**birddogcmsetup_get**](docs/DefaultApi.md#birddogcmsetup_get) | **GET** /birddogcmsetup | Retrieve Colour Matrix Settings
*DefaultApi* | [**birddogcmsetup_post**](docs/DefaultApi.md#birddogcmsetup_post) | **POST** /birddogcmsetup | Set Colour Matrix Settings
*DefaultApi* | [**birddogdetsetup_get**](docs/DefaultApi.md#birddogdetsetup_get) | **GET** /birddogdetsetup | Retrieve Detail Settings
*DefaultApi* | [**birddogdetsetup_post**](docs/DefaultApi.md#birddogdetsetup_post) | **POST** /birddogdetsetup | Set Detail Settings
*DefaultApi* | [**birddogexpsetup_get**](docs/DefaultApi.md#birddogexpsetup_get) | **GET** /birddogexpsetup | Retrieve Exposure Settings
*DefaultApi* | [**birddogexpsetup_post**](docs/DefaultApi.md#birddogexpsetup_post) | **POST** /birddogexpsetup | Set Exposure Settings
*DefaultApi* | [**birddogexternalsetup_get**](docs/DefaultApi.md#birddogexternalsetup_get) | **GET** /birddogexternalsetup | Retrieve External Settings
*DefaultApi* | [**birddogexternalsetup_post**](docs/DefaultApi.md#birddogexternalsetup_post) | **POST** /birddogexternalsetup | Set External Settings
*DefaultApi* | [**birddoggammasetup_get**](docs/DefaultApi.md#birddoggammasetup_get) | **GET** /birddoggammasetup | Retrieve Gamma Settings
*DefaultApi* | [**birddoggammasetup_post**](docs/DefaultApi.md#birddoggammasetup_post) | **POST** /birddoggammasetup | Set Gamma Settings
*DefaultApi* | [**birddogpicsetup_get**](docs/DefaultApi.md#birddogpicsetup_get) | **GET** /birddogpicsetup | Retrieve Picture Settings
*DefaultApi* | [**birddogpicsetup_post**](docs/DefaultApi.md#birddogpicsetup_post) | **POST** /birddogpicsetup | Set Picture Settings
*DefaultApi* | [**birddogptzsetup_get**](docs/DefaultApi.md#birddogptzsetup_get) | **GET** /birddogptzsetup | Retrieve PTZ Settings
*DefaultApi* | [**birddogptzsetup_post**](docs/DefaultApi.md#birddogptzsetup_post) | **POST** /birddogptzsetup | Set PTZ Settings
*DefaultApi* | [**birddogsil2codec_get**](docs/DefaultApi.md#birddogsil2codec_get) | **GET** /birddogsil2codec | Retrieve Silicon2 Codec Settings
*DefaultApi* | [**birddogsil2codec_post**](docs/DefaultApi.md#birddogsil2codec_post) | **POST** /birddogsil2codec | Set Silicon2 Codec Settings
*DefaultApi* | [**birddogsil2enc_get**](docs/DefaultApi.md#birddogsil2enc_get) | **GET** /birddogsil2enc | Retrieve Silicon2 Encode Settings
*DefaultApi* | [**birddogsil2enc_post**](docs/DefaultApi.md#birddogsil2enc_post) | **POST** /birddogsil2enc | Set Silicon2 Encode Settings
*DefaultApi* | [**birddogwbsetup_get**](docs/DefaultApi.md#birddogwbsetup_get) | **GET** /birddogwbsetup | Retrieve White Balance Settings
*DefaultApi* | [**birddogwbsetup_post**](docs/DefaultApi.md#birddogwbsetup_post) | **POST** /birddogwbsetup | Set White Balance Settings
*DefaultApi* | [**capture_get**](docs/DefaultApi.md#capture_get) | **GET** /capture | Capture screensaver frame for Encode/Decode
*DefaultApi* | [**connect_to_get**](docs/DefaultApi.md#connect_to_get) | **GET** /connectTo | Retrieve connected NDI Source info (sourceName)
*DefaultApi* | [**connect_to_post**](docs/DefaultApi.md#connect_to_post) | **POST** /connectTo | Connect to NDI Source
*DefaultApi* | [**decode_transport_get**](docs/DefaultApi.md#decode_transport_get) | **GET** /decodeTransport | Retrieve NDI network settings
*DefaultApi* | [**decode_transport_post**](docs/DefaultApi.md#decode_transport_post) | **POST** /decodeTransport | Set NDI network settings
*DefaultApi* | [**decodesetup_get**](docs/DefaultApi.md#decodesetup_get) | **GET** /decodesetup | Retrieve Decode Settings
*DefaultApi* | [**decodesetup_post**](docs/DefaultApi.md#decodesetup_post) | **POST** /decodesetup | Set Decode Settings
*DefaultApi* | [**decodestatus_get**](docs/DefaultApi.md#decodestatus_get) | **GET** /decodestatus | Retrieve connected NDI Source Status
*DefaultApi* | [**encode_transport_get**](docs/DefaultApi.md#encode_transport_get) | **GET** /encodeTransport | Retrieve current NDI Network Settings
*DefaultApi* | [**encode_transport_post**](docs/DefaultApi.md#encode_transport_post) | **POST** /encodeTransport | Set NDI Network Settings
*DefaultApi* | [**encodesetup_get**](docs/DefaultApi.md#encodesetup_get) | **GET** /encodesetup | Retrieve Encode Settings
*DefaultApi* | [**encodesetup_post**](docs/DefaultApi.md#encodesetup_post) | **POST** /encodesetup | Set Encode Settings
*DefaultApi* | [**hostname_get**](docs/DefaultApi.md#hostname_get) | **GET** /hostname | Retrieve device hostname
*DefaultApi* | [**list_get**](docs/DefaultApi.md#list_get) | **GET** /List | Retrieve List of active NDI Sources on the Network
*DefaultApi* | [**n_di_dis_server_get**](docs/DefaultApi.md#n_di_dis_server_get) | **GET** /NDIDisServer | Retrieve NDI Discovery server info
*DefaultApi* | [**n_di_dis_server_post**](docs/DefaultApi.md#n_di_dis_server_post) | **POST** /NDIDisServer | Set NDI Discovery server info
*DefaultApi* | [**n_di_grp_name_get**](docs/DefaultApi.md#n_di_grp_name_get) | **GET** /NDIGrpName | Retrieve GroupName
*DefaultApi* | [**n_di_grp_name_post**](docs/DefaultApi.md#n_di_grp_name_post) | **POST** /NDIGrpName | Set GroupName
*DefaultApi* | [**n_di_off_sn_src_get**](docs/DefaultApi.md#n_di_off_sn_src_get) | **GET** /NDIOffSnSrc | Retrieve Off Subnet IP List
*DefaultApi* | [**n_di_off_sn_src_post**](docs/DefaultApi.md#n_di_off_sn_src_post) | **POST** /NDIOffSnSrc | Set Off Subnet IP
*DefaultApi* | [**operationmode_get**](docs/DefaultApi.md#operationmode_get) | **GET** /operationmode | Retrieve current operation mode of your BirdDog device (encode/decode)
*DefaultApi* | [**operationmode_post**](docs/DefaultApi.md#operationmode_post) | **POST** /operationmode | Set base operation modes of your BirdDog device
*DefaultApi* | [**reboot_get**](docs/DefaultApi.md#reboot_get) | **GET** /reboot | Reboot BirdDog device
*DefaultApi* | [**reboot_post**](docs/DefaultApi.md#reboot_post) | **POST** /reboot | Reboot BirdDog device
*DefaultApi* | [**recall_post**](docs/DefaultApi.md#recall_post) | **POST** /recall | Recall Preset
*DefaultApi* | [**refresh_get**](docs/DefaultApi.md#refresh_get) | **GET** /refresh | Refresh NDI Source List
*DefaultApi* | [**refresh_post**](docs/DefaultApi.md#refresh_post) | **POST** /refresh | Refresh NDI Source List
*DefaultApi* | [**reset_get**](docs/DefaultApi.md#reset_get) | **GET** /reset | Reset NDI Source List
*DefaultApi* | [**reset_post**](docs/DefaultApi.md#reset_post) | **POST** /reset | Reset NDI Source List
*DefaultApi* | [**restart_get**](docs/DefaultApi.md#restart_get) | **GET** /restart | Restart video subsystem on device
*DefaultApi* | [**restart_post**](docs/DefaultApi.md#restart_post) | **POST** /restart | Restart video subsystem on device
*DefaultApi* | [**save_post**](docs/DefaultApi.md#save_post) | **POST** /save | Save Preset
*DefaultApi* | [**version_get**](docs/DefaultApi.md#version_get) | **GET** /version | Hardware version number query
*DefaultApi* | [**videooutputinterface_get**](docs/DefaultApi.md#videooutputinterface_get) | **GET** /videooutputinterface | Retrieve current videooutputinterface mode of your BirdDog device (sdi/hdmi) or (LowLatency/NormalMode)
*DefaultApi* | [**videooutputinterface_post**](docs/DefaultApi.md#videooutputinterface_post) | **POST** /videooutputinterface | Set base videooutputinterface mode of your BirdDog device

## Documentation For Models

 - [AnalogaudiosetupPostRequest](docs/AnalogaudiosetupPostRequest.md)
 - [BirddogadvancesetupPostRequest](docs/BirddogadvancesetupPostRequest.md)
 - [BirddogcmsetupPostRequest](docs/BirddogcmsetupPostRequest.md)
 - [BirddogdetsetupPostRequest](docs/BirddogdetsetupPostRequest.md)
 - [BirddogexpsetupPostRequest](docs/BirddogexpsetupPostRequest.md)
 - [BirddogexternalsetupPostRequest](docs/BirddogexternalsetupPostRequest.md)
 - [BirddoggammasetupPostRequest](docs/BirddoggammasetupPostRequest.md)
 - [BirddogpicsetupPostRequest](docs/BirddogpicsetupPostRequest.md)
 - [BirddogptzsetupPostRequest](docs/BirddogptzsetupPostRequest.md)
 - [Birddogsil2codecPostRequest](docs/Birddogsil2codecPostRequest.md)
 - [Birddogsil2encPostRequest](docs/Birddogsil2encPostRequest.md)
 - [BirddogwbsetupPostRequest](docs/BirddogwbsetupPostRequest.md)
 - [ConnectToPostRequest](docs/ConnectToPostRequest.md)
 - [DecodeTransportPostRequest](docs/DecodeTransportPostRequest.md)
 - [DecodesetupPostRequest](docs/DecodesetupPostRequest.md)
 - [EncodeTransportPostRequest](docs/EncodeTransportPostRequest.md)
 - [EncodesetupPostRequest](docs/EncodesetupPostRequest.md)
 - [NdiDisServerPostRequest](docs/NdiDisServerPostRequest.md)
 - [RecallPostRequest](docs/RecallPostRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss what you would like to change.

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for more details.