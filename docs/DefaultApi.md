# \DefaultApi

All URIs are relative to *http://192.168.2.197:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**about_get**](DefaultApi.md#about_get) | **GET** /about | Retrieve basic information from your BirdDog device
[**analogaudiosetup_get**](DefaultApi.md#analogaudiosetup_get) | **GET** /analogaudiosetup | Retrieve Audio Settings
[**analogaudiosetup_post**](DefaultApi.md#analogaudiosetup_post) | **POST** /analogaudiosetup | Set Audio Settings
[**birddogadvancesetup_get**](DefaultApi.md#birddogadvancesetup_get) | **GET** /birddogadvancesetup | Retrieve Advanced Settings
[**birddogadvancesetup_post**](DefaultApi.md#birddogadvancesetup_post) | **POST** /birddogadvancesetup | Set Advanced Settings
[**birddogcmsetup_get**](DefaultApi.md#birddogcmsetup_get) | **GET** /birddogcmsetup | Retrieve Colour Matrix Settings
[**birddogcmsetup_post**](DefaultApi.md#birddogcmsetup_post) | **POST** /birddogcmsetup | Set Colour Matrix Settings
[**birddogdetsetup_get**](DefaultApi.md#birddogdetsetup_get) | **GET** /birddogdetsetup | Retrieve Detail Settings
[**birddogdetsetup_post**](DefaultApi.md#birddogdetsetup_post) | **POST** /birddogdetsetup | Set Detail Settings
[**birddogexpsetup_get**](DefaultApi.md#birddogexpsetup_get) | **GET** /birddogexpsetup | Retrieve Exposure Settings
[**birddogexpsetup_post**](DefaultApi.md#birddogexpsetup_post) | **POST** /birddogexpsetup | Set Exposure Settings
[**birddogexternalsetup_get**](DefaultApi.md#birddogexternalsetup_get) | **GET** /birddogexternalsetup | Retrieve External Settings
[**birddogexternalsetup_post**](DefaultApi.md#birddogexternalsetup_post) | **POST** /birddogexternalsetup | Set External Settings
[**birddoggammasetup_get**](DefaultApi.md#birddoggammasetup_get) | **GET** /birddoggammasetup | Retrieve Gamma Settings
[**birddoggammasetup_post**](DefaultApi.md#birddoggammasetup_post) | **POST** /birddoggammasetup | Set Gamma Settings
[**birddogpicsetup_get**](DefaultApi.md#birddogpicsetup_get) | **GET** /birddogpicsetup | Retrieve Picture Settings
[**birddogpicsetup_post**](DefaultApi.md#birddogpicsetup_post) | **POST** /birddogpicsetup | Set Picture Settings
[**birddogptzsetup_get**](DefaultApi.md#birddogptzsetup_get) | **GET** /birddogptzsetup | Retrieve PTZ Settings
[**birddogptzsetup_post**](DefaultApi.md#birddogptzsetup_post) | **POST** /birddogptzsetup | Set PTZ Settings
[**birddogsil2codec_get**](DefaultApi.md#birddogsil2codec_get) | **GET** /birddogsil2codec | Retrieve Silicon2 Codec Settings
[**birddogsil2codec_post**](DefaultApi.md#birddogsil2codec_post) | **POST** /birddogsil2codec | Set Silicon2 Codec Settings
[**birddogsil2enc_get**](DefaultApi.md#birddogsil2enc_get) | **GET** /birddogsil2enc | Retrieve Silicon2 Encode Settings
[**birddogsil2enc_post**](DefaultApi.md#birddogsil2enc_post) | **POST** /birddogsil2enc | Set Silicon2 Encode Settings
[**birddogwbsetup_get**](DefaultApi.md#birddogwbsetup_get) | **GET** /birddogwbsetup | Retrieve White Balance Settings
[**birddogwbsetup_post**](DefaultApi.md#birddogwbsetup_post) | **POST** /birddogwbsetup | Set White Balance Settings
[**capture_get**](DefaultApi.md#capture_get) | **GET** /capture | Capture screensaver frame for Encode/Decode
[**connect_to_get**](DefaultApi.md#connect_to_get) | **GET** /connectTo | Retrieve connected NDI Source info (sourceName)
[**connect_to_post**](DefaultApi.md#connect_to_post) | **POST** /connectTo | Connects to NDI Source
[**decode_transport_get**](DefaultApi.md#decode_transport_get) | **GET** /decodeTransport | Retrieve NDI network settings
[**decode_transport_post**](DefaultApi.md#decode_transport_post) | **POST** /decodeTransport | Set NDI network settings
[**decodesetup_get**](DefaultApi.md#decodesetup_get) | **GET** /decodesetup | Retrieve Decode Settings
[**decodesetup_post**](DefaultApi.md#decodesetup_post) | **POST** /decodesetup | Set Decode Settings
[**decodestatus_get**](DefaultApi.md#decodestatus_get) | **GET** /decodestatus | Retrieve connected NDI Source Status
[**encode_transport_get**](DefaultApi.md#encode_transport_get) | **GET** /encodeTransport | Retrieve current NDI Network Settings
[**encode_transport_post**](DefaultApi.md#encode_transport_post) | **POST** /encodeTransport | Set NDI Network Settings
[**encodesetup_get**](DefaultApi.md#encodesetup_get) | **GET** /encodesetup | Retrieve Encode Settings
[**encodesetup_post**](DefaultApi.md#encodesetup_post) | **POST** /encodesetup | Set Encode Settings
[**hostname_get**](DefaultApi.md#hostname_get) | **GET** /hostname | Retrieve device hostname
[**list_get**](DefaultApi.md#list_get) | **GET** /List | Retrieve List of active NDI Sources on the Network
[**n_di_dis_server_get**](DefaultApi.md#n_di_dis_server_get) | **GET** /NDIDisServer | Retrieve NDI Discovery server info
[**n_di_dis_server_post**](DefaultApi.md#n_di_dis_server_post) | **POST** /NDIDisServer | Set NDI Discovery server info
[**n_di_grp_name_get**](DefaultApi.md#n_di_grp_name_get) | **GET** /NDIGrpName | Retrieve GroupName
[**n_di_grp_name_post**](DefaultApi.md#n_di_grp_name_post) | **POST** /NDIGrpName | Set GroupName
[**n_di_off_sn_src_get**](DefaultApi.md#n_di_off_sn_src_get) | **GET** /NDIOffSnSrc | Retrieve Off Subnet IP List
[**n_di_off_sn_src_post**](DefaultApi.md#n_di_off_sn_src_post) | **POST** /NDIOffSnSrc | Set Off Subnet IP
[**operationmode_get**](DefaultApi.md#operationmode_get) | **GET** /operationmode | Retrieve current operation mode of your BirdDog device (encode/decode)
[**operationmode_post**](DefaultApi.md#operationmode_post) | **POST** /operationmode | Set base operation modes of your BirdDog device
[**reboot_get**](DefaultApi.md#reboot_get) | **GET** /reboot | Reboot BirdDog device
[**reboot_post**](DefaultApi.md#reboot_post) | **POST** /reboot | Reboot BirdDog device
[**recall_post**](DefaultApi.md#recall_post) | **POST** /recall | Recall Preset
[**refresh_get**](DefaultApi.md#refresh_get) | **GET** /refresh | Refresh NDI Source List
[**refresh_post**](DefaultApi.md#refresh_post) | **POST** /refresh | Refresh NDI Source List
[**reset_get**](DefaultApi.md#reset_get) | **GET** /reset | Reset NDI Source List
[**reset_post**](DefaultApi.md#reset_post) | **POST** /reset | Reset NDI Source List
[**restart_get**](DefaultApi.md#restart_get) | **GET** /restart | Restart video subsystem on device
[**restart_post**](DefaultApi.md#restart_post) | **POST** /restart | Restart video subsystem on device
[**save_post**](DefaultApi.md#save_post) | **POST** /save | Save Preset
[**version_get**](DefaultApi.md#version_get) | **GET** /version | Hardware version number query
[**videooutputinterface_get**](DefaultApi.md#videooutputinterface_get) | **GET** /videooutputinterface | Retrieve current video output interface mode of your BirdDog device (sdi/hdmi) or (LowLatency/NormalMode)
[**videooutputinterface_post**](DefaultApi.md#videooutputinterface_post) | **POST** /videooutputinterface | Set base video output interface mode of your BirdDog device



## about_get

> models::AboutGet200Response about_get(firmware_version, format, host_name, ip_address, network_config_method, serial_number, status)
Retrieve basic information from your BirdDog device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firmware_version** | Option<**String**> | Device manufacturer 'BIRDDOG' |  |
**format** | Option<**String**> | Device manufacturer extended 'BIRDDOG' |  |
**host_name** | Option<**String**> | Hardware Revision '1.0' |  |
**ip_address** | Option<**String**> | Device IP Address 'xxx.xxx.xxx.xxx' |  |
**network_config_method** | Option<**String**> | IP Address method 'DHCP' 'STATIC' |  |
**serial_number** | Option<**String**> | Device Hostname (AVAHI) 'birddog-nnnnn' |  |
**status** | Option<**String**> | Current device status 'ONLINE' 'OFFLINE' 'CAMERA INITIALIZING' 'NO VIDEO' |  |

### Return type

[**models::AboutGet200Response**](_about_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analogaudiosetup_get

> models::AnalogaudiosetupGet200Response analogaudiosetup_get(analog_audio_in_gain, analog_audio_out_gain, analog_audiooutputselect)
Retrieve Audio Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analog_audio_in_gain** | Option<**String**> | Audio in gain Range 0 to 100 |  |
**analog_audio_out_gain** | Option<**String**> | Audio out gain Range 0 to 100 |  |
**analog_audiooutputselect** | Option<**String**> | DecodeMain, DecodeComms, DecodeLoop |  |

### Return type

[**models::AnalogaudiosetupGet200Response**](_analogaudiosetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## analogaudiosetup_post

> models::AnalogaudiosetupGet200Response analogaudiosetup_post(analogaudiosetup_post_request, analog_audio_in_gain, analog_audio_out_gain, analog_audiooutputselect)
Set Audio Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analogaudiosetup_post_request** | [**AnalogaudiosetupPostRequest**](AnalogaudiosetupPostRequest.md) |  | [required] |
**analog_audio_in_gain** | Option<**String**> | Audio in gain Range 0 to 100 |  |
**analog_audio_out_gain** | Option<**String**> | Audio out gain Range 0 to 100 |  |
**analog_audiooutputselect** | Option<**String**> | DecodeMain, DecodeComms, DecodeLoop |  |

### Return type

[**models::AnalogaudiosetupGet200Response**](_analogaudiosetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogadvancesetup_get

> models::BirddogadvancesetupGet200Response birddogadvancesetup_get(brightness, brightness_comp, comp_level, gamma_offset, high_resolution, video_enhancement)
Retrieve Advanced Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**brightness** | Option<**String**> | Range (0 to 6) |  |
**brightness_comp** | Option<**String**> | VERY DARK, DARK, STANDARD, BRIGHT |  |
**comp_level** | Option<**String**> | LOW, MID, HIGH |  |
**gamma_offset** | Option<**String**> | Range (16 to 64) |  |
**high_resolution** | Option<**String**> | On, Off |  |
**video_enhancement** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddogadvancesetupGet200Response**](_birddogadvancesetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogadvancesetup_post

> models::BirddogadvancesetupGet200Response birddogadvancesetup_post(birddogadvancesetup_get200_response, brightness, brightness_comp, comp_level, gamma_offset, high_resolution, video_enhancement)
Set Advanced Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogadvancesetup_get200_response** | [**BirddogadvancesetupGet200Response**](BirddogadvancesetupGet200Response.md) |  | [required] |
**brightness** | Option<**String**> | Range (0 to 6) |  |
**brightness_comp** | Option<**String**> | VERY DARK, DARK, STANDARD, BRIGHT |  |
**comp_level** | Option<**String**> | LOW, MID, HIGH |  |
**gamma_offset** | Option<**String**> | Range (16 to 64) |  |
**high_resolution** | Option<**String**> | On, Off |  |
**video_enhancement** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddogadvancesetupGet200Response**](_birddogadvancesetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogcmsetup_get

> models::BirddogcmsetupGet200Response birddogcmsetup_get(blue_gain, blue_hue, colour_gain, cyan_gain, cyan_hue, green_gain, green_hue, hue_phase, mag_gain, mag_hue, red_gain, red_hue, yellow_gain, yellow_hue)
Retrieve Colour Matrix Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blue_gain** | Option<**String**> | Range (0 to 64) |  |
**blue_hue** | Option<**String**> | Range (0 to 64) |  |
**colour_gain** | Option<**String**> | Range (0 to 255) |  |
**cyan_gain** | Option<**String**> | Range (0 to 64) |  |
**cyan_hue** | Option<**String**> | Range (0 to 64) |  |
**green_gain** | Option<**String**> | Range (0 to 64) |  |
**green_hue** | Option<**String**> | Range (0 to 64) |  |
**hue_phase** | Option<**String**> | Range (0 to 255) |  |
**mag_gain** | Option<**String**> | Range (0 to 64) |  |
**mag_hue** | Option<**String**> | Range (0 to 64) |  |
**red_gain** | Option<**String**> | Range (0 to 64) |  |
**red_hue** | Option<**String**> | Range (0 to 64) |  |
**yellow_gain** | Option<**String**> | Range (0 to 64) |  |
**yellow_hue** | Option<**String**> | Range (0 to 64) |  |

### Return type

[**models::BirddogcmsetupGet200Response**](_birddogcmsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogcmsetup_post

> models::BirddogcmsetupGet200Response birddogcmsetup_post(birddogcmsetup_get200_response, blue_gain, blue_hue, colour_gain, cyan_gain, cyan_hue, green_gain, green_hue, hue_phase, mag_gain, mag_hue, red_gain, red_hue, yellow_gain, yellow_hue)
Set Colour Matrix Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogcmsetup_get200_response** | [**BirddogcmsetupGet200Response**](BirddogcmsetupGet200Response.md) |  | [required] |
**blue_gain** | Option<**String**> | Range (0 to 64) |  |
**blue_hue** | Option<**String**> | Range (0 to 64) |  |
**colour_gain** | Option<**String**> | Range (0 to 255) |  |
**cyan_gain** | Option<**String**> | Range (0 to 64) |  |
**cyan_hue** | Option<**String**> | Range (0 to 64) |  |
**green_gain** | Option<**String**> | Range (0 to 64) |  |
**green_hue** | Option<**String**> | Range (0 to 64) |  |
**hue_phase** | Option<**String**> | Range (0 to 255) |  |
**mag_gain** | Option<**String**> | Range (0 to 64) |  |
**mag_hue** | Option<**String**> | Range (0 to 64) |  |
**red_gain** | Option<**String**> | Range (0 to 64) |  |
**red_hue** | Option<**String**> | Range (0 to 64) |  |
**yellow_gain** | Option<**String**> | Range (0 to 64) |  |
**yellow_hue** | Option<**String**> | Range (0 to 64) |  |

### Return type

[**models::BirddogcmsetupGet200Response**](_birddogcmsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogdetsetup_get

> models::BirddogdetsetupGet200Response birddogdetsetup_get(bandwidth, bw_balance, crispening, detail, high_light_detail, hv_balance, limit, super_low)
Retrieve Detail Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bandwidth** | Option<**String**> | DEFAULT, LOW, MIDDLE, HIGH, WIDE |  |
**bw_balance** | Option<**String**> | TYPE1, TYPE2, TYPE3, TYPE4, TYPE5 |  |
**crispening** | Option<**String**> | Range (0 to 7) |  |
**detail** | Option<**String**> | On, Off |  |
**high_light_detail** | Option<**String**> | Range (0 to 4) |  |
**hv_balance** | Option<**String**> | Range (-2 to 2) |  |
**limit** | Option<**String**> | Range (0 to 7) |  |
**super_low** | Option<**String**> | Range (0 to 7) |  |

### Return type

[**models::BirddogdetsetupGet200Response**](_birddogdetsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogdetsetup_post

> models::BirddogdetsetupGet200Response birddogdetsetup_post(birddogdetsetup_get200_response, bandwidth, bw_balance, crispening, detail, high_light_detail, hv_balance, limit, super_low)
Set Detail Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogdetsetup_get200_response** | [**BirddogdetsetupGet200Response**](BirddogdetsetupGet200Response.md) |  | [required] |
**bandwidth** | Option<**String**> | DEFAULT, LOW, MIDDLE, HIGH, WIDE |  |
**bw_balance** | Option<**String**> | TYPE1, TYPE2, TYPE3, TYPE4, TYPE5 |  |
**crispening** | Option<**String**> | Range (0 to 7) |  |
**detail** | Option<**String**> | On, Off |  |
**high_light_detail** | Option<**String**> | Range (0 to 4) |  |
**hv_balance** | Option<**String**> | Range (-2 to 2) |  |
**limit** | Option<**String**> | Range (0 to 7) |  |
**super_low** | Option<**String**> | Range (0 to 7) |  |

### Return type

[**models::BirddogdetsetupGet200Response**](_birddogdetsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogexpsetup_get

> models::BirddogexpsetupGet200Response birddogexpsetup_get(ae_response, back_light, bright_level, exp_comp_en, exp_comp_lvl, exp_mode, gain_level, gain_limit, gain_point, gain_point_position, high_sensitivity, iris_level, shutter_control_overwrite, shutter_max_speed, shutter_min_speed, shutter_speed, shutter_speed_overwrite, slow_shutter_en, slow_shutter_limit, spotlight)
Retrieve Exposure Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ae_response** | Option<**String**> | Range (1 to 48) |  |
**back_light** | Option<**String**> | On, Off |  |
**bright_level** | Option<**String**> | Range (0, 5 to 31) |  |
**exp_comp_en** | Option<**String**> | On, Off |  |
**exp_comp_lvl** | Option<**String**> | Range (0 to 14, -128 to 127) |  |
**exp_mode** | Option<**String**> | FULL-AUTO, MANUAL, SHUTTER-PRI, IRIS-PRI, BRIGHT |  |
**gain_level** | Option<**String**> | Range (1 to GainLimit) |  |
**gain_limit** | Option<**String**> | Range (4 to 15) |  |
**gain_point** | Option<**String**> | On, Off |  |
**gain_point_position** | Option<**String**> | Range (1 to 13) |  |
**high_sensitivity** | Option<**String**> | On, Off |  |
**iris_level** | Option<**String**> | Range (0, 5 to 17) |  |
**shutter_control_overwrite** | Option<**String**> | On, Off |  |
**shutter_max_speed** | Option<**String**> | Range (20 to 33) |  |
**shutter_min_speed** | Option<**String**> | Range (16 to ShutterMaxSpeed) |  |
**shutter_speed** | Option<**String**> | Range (0 to 21) |  |
**shutter_speed_overwrite** | Option<**String**> | Range (30 to 110) |  |
**slow_shutter_en** | Option<**String**> | On, Off |  |
**slow_shutter_limit** | Option<**String**> | Range (1 to 6) |  |
**spotlight** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddogexpsetupGet200Response**](_birddogexpsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogexpsetup_post

> models::BirddogexpsetupGet200Response birddogexpsetup_post(birddogexpsetup_get200_response, ae_response, back_light, bright_level, exp_comp_en, exp_comp_lvl, exp_mode, gain_level, gain_limit, gain_point, gain_point_position, high_sensitivity, iris_level, shutter_control_overwrite, shutter_max_speed, shutter_min_speed, shutter_speed, shutter_speed_overwrite, slow_shutter_en, slow_shutter_limit, spotlight)
Set Exposure Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogexpsetup_get200_response** | [**BirddogexpsetupGet200Response**](BirddogexpsetupGet200Response.md) |  | [required] |
**ae_response** | Option<**String**> | Range (1 to 48) |  |
**back_light** | Option<**String**> | On, Off |  |
**bright_level** | Option<**String**> | Range (0, 5 to 31) |  |
**exp_comp_en** | Option<**String**> | On, Off |  |
**exp_comp_lvl** | Option<**String**> | Range (0 to 14, -128 to 127) |  |
**exp_mode** | Option<**String**> | FULL-AUTO, MANUAL, SHUTTER-PRI, IRIS-PRI, BRIGHT |  |
**gain_level** | Option<**String**> | Range (1 to GainLimit) |  |
**gain_limit** | Option<**String**> | Range (4 to 15) |  |
**gain_point** | Option<**String**> | On, Off |  |
**gain_point_position** | Option<**String**> | Range (1 to 13) |  |
**high_sensitivity** | Option<**String**> | On, Off |  |
**iris_level** | Option<**String**> | Range (0, 5 to 17) |  |
**shutter_control_overwrite** | Option<**String**> | On, Off |  |
**shutter_max_speed** | Option<**String**> | Range (20 to 33) |  |
**shutter_min_speed** | Option<**String**> | Range (16 to ShutterMaxSpeed) |  |
**shutter_speed** | Option<**String**> | Range (0 to 21) |  |
**shutter_speed_overwrite** | Option<**String**> | Range (30 to 110) |  |
**slow_shutter_en** | Option<**String**> | On, Off |  |
**slow_shutter_limit** | Option<**String**> | Range (1 to 6) |  |
**spotlight** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddogexpsetupGet200Response**](_birddogexpsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogexternalsetup_get

> models::BirddogexternalsetupGet200Response birddogexternalsetup_get(aux, rain_wiper, v12v_out)
Retrieve External Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aux** | Option<**String**> | On, Off |  |
**rain_wiper** | Option<**String**> | On, Off |  |
**v12v_out** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddogexternalsetupGet200Response**](_birddogexternalsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogexternalsetup_post

> models::BirddogexternalsetupGet200Response birddogexternalsetup_post(birddogexternalsetup_get200_response, aux, rain_wiper, v12v_out)
Set External Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogexternalsetup_get200_response** | [**BirddogexternalsetupGet200Response**](BirddogexternalsetupGet200Response.md) |  | [required] |
**aux** | Option<**String**> | On, Off |  |
**rain_wiper** | Option<**String**> | On, Off |  |
**v12v_out** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddogexternalsetupGet200Response**](_birddogexternalsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddoggammasetup_get

> models::BirddoggammasetupGet200Response birddoggammasetup_get(black_gamma_level, black_level, black_level_range, effect, level, offset, pattern, pattern_fine, settings, visibility_enhancer)
Retrieve Gamma Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**black_gamma_level** | Option<**String**> | Range (0 to 14) |  |
**black_level** | Option<**String**> | Range (0 to 96) |  |
**black_level_range** | Option<**String**> | LOW, MID, HIGH |  |
**effect** | Option<**String**> | Range (-3 to 3) |  |
**level** | Option<**String**> | Range (0 to 14) |  |
**offset** | Option<**String**> | Range (-64 to 64) |  |
**pattern** | Option<**String**> | Range (0 to 512) |  |
**pattern_fine** | Option<**String**> | Range (0 to 9) |  |
**settings** | Option<**String**> | PATTERN, STANDARD, STRAIGHT |  |
**visibility_enhancer** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddoggammasetupGet200Response**](_birddoggammasetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddoggammasetup_post

> models::BirddoggammasetupGet200Response birddoggammasetup_post(birddoggammasetup_get200_response, black_gamma_level, black_level, black_level_range, effect, level, offset, pattern, pattern_fine, settings, visibility_enhancer)
Set Gamma Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddoggammasetup_get200_response** | [**BirddoggammasetupGet200Response**](BirddoggammasetupGet200Response.md) |  | [required] |
**black_gamma_level** | Option<**String**> | Range (0 to 14) |  |
**black_level** | Option<**String**> | Range (0 to 96) |  |
**black_level_range** | Option<**String**> | LOW, MID, HIGH |  |
**effect** | Option<**String**> | Range (-3 to 3) |  |
**level** | Option<**String**> | Range (0 to 14) |  |
**offset** | Option<**String**> | Range (-64 to 64) |  |
**pattern** | Option<**String**> | Range (0 to 512) |  |
**pattern_fine** | Option<**String**> | Range (0 to 9) |  |
**settings** | Option<**String**> | PATTERN, STANDARD, STRAIGHT |  |
**visibility_enhancer** | Option<**String**> | On, Off |  |

### Return type

[**models::BirddoggammasetupGet200Response**](_birddoggammasetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogpicsetup_get

> models::BirddogpicsetupGet200Response birddogpicsetup_get(back_light_com, chrome_suppress, color, contrast, effect, flip, gamma, highlight_comp, highlight_comp_mask, hue, ir_cut_filter, mirror, noise_reduction, sharpness, stabilizer, twodnr, three_dnr, wide_dynamic_range, low_latency, nd_filter)
Retrieve Picture Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**back_light_com** | Option<**String**> | On, Off |  |
**chrome_suppress** | Option<**String**> | OFF, LOW, MEDIUM, HIGH |  |
**color** | Option<**String**> | Range (0 to 15) |  |
**contrast** | Option<**String**> | Range (0 to 15) |  |
**effect** | Option<**String**> | BW, Off |  |
**flip** | Option<**String**> | On, Off |  |
**gamma** | Option<**String**> | Range (0 to 4) |  |
**highlight_comp** | Option<**String**> | OFF, LOW, MEDIUM, HIGH |  |
**highlight_comp_mask** | Option<**String**> | Range (0 to 15) |  |
**hue** | Option<**String**> | Range (0 to 15) |  |
**ir_cut_filter** | Option<**String**> | Auto, On, Off |  |
**mirror** | Option<**String**> | On, Off |  |
**noise_reduction** | Option<**String**> | Range (Off, 1 to 5) |  |
**sharpness** | Option<**String**> | Range (-128 to 127) |  |
**stabilizer** | Option<**String**> | On, Off |  |
**twodnr** | Option<**String**> | Range (Off, 1 to 5) |  |
**three_dnr** | Option<**String**> | Range (Off, 1 to 5) |  |
**wide_dynamic_range** | Option<**String**> | Range (Off, 1 to 6) |  |
**low_latency** | Option<**String**> | On, Off |  |
**nd_filter** | Option<**String**> | Range (0 to 3) |  |

### Return type

[**models::BirddogpicsetupGet200Response**](_birddogpicsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogpicsetup_post

> models::BirddogpicsetupGet200Response birddogpicsetup_post(birddogpicsetup_get200_response, back_light_com, chrome_suppress, color, contrast, effect, flip, gamma, highlight_comp, highlight_comp_mask, hue, ir_cut_filter, mirror, noise_reduction, sharpness, stabilizer, twodnr, three_dnr, wide_dynamic_range, low_latency, nd_filter)
Set Picture Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogpicsetup_get200_response** | [**BirddogpicsetupGet200Response**](BirddogpicsetupGet200Response.md) |  | [required] |
**back_light_com** | Option<**String**> | On, Off |  |
**chrome_suppress** | Option<**String**> | OFF, LOW, MEDIUM, HIGH |  |
**color** | Option<**String**> | Range (0 to 15) |  |
**contrast** | Option<**String**> | Range (0 to 15) |  |
**effect** | Option<**String**> | BW, Off |  |
**flip** | Option<**String**> | On, Off |  |
**gamma** | Option<**String**> | Range (0 to 4) |  |
**highlight_comp** | Option<**String**> | OFF, LOW, MEDIUM, HIGH |  |
**highlight_comp_mask** | Option<**String**> | Range (0 to 15) |  |
**hue** | Option<**String**> | Range (0 to 15) |  |
**ir_cut_filter** | Option<**String**> | Auto, On, Off |  |
**mirror** | Option<**String**> | On, Off |  |
**noise_reduction** | Option<**String**> | Range (Off, 1 to 5) |  |
**sharpness** | Option<**String**> | Range (-128 to 127) |  |
**stabilizer** | Option<**String**> | On, Off |  |
**twodnr** | Option<**String**> | Range (Off, 1 to 5) |  |
**three_dnr** | Option<**String**> | Range (Off, 1 to 5) |  |
**wide_dynamic_range** | Option<**String**> | Range (Off, 1 to 6) |  |
**low_latency** | Option<**String**> | On, Off |  |
**nd_filter** | Option<**String**> | Range (0 to 3) |  |

### Return type

[**models::BirddogpicsetupGet200Response**](_birddogpicsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogptzsetup_get

> models::BirddogptzsetupGet200Response birddogptzsetup_get(pan_speed, tilt_speed, zoom_speed)
Retrieve PTZ Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pan_speed** | Option<**String**> | Range from 0 to 21 |  |
**tilt_speed** | Option<**String**> | Range from 0 to 18 |  |
**zoom_speed** | Option<**String**> | Range from 0 to 7 |  |

### Return type

[**models::BirddogptzsetupGet200Response**](_birddogptzsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogptzsetup_post

> models::BirddogptzsetupGet200Response birddogptzsetup_post(birddogptzsetup_get200_response, pan_speed, tilt_speed, zoom_speed)
Set PTZ Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogptzsetup_get200_response** | [**BirddogptzsetupGet200Response**](BirddogptzsetupGet200Response.md) |  | [required] |
**pan_speed** | Option<**String**> | Range from 0 to 21 |  |
**tilt_speed** | Option<**String**> | Range from 0 to 18 |  |
**zoom_speed** | Option<**String**> | Range from 0 to 7 |  |

### Return type

[**models::BirddogptzsetupGet200Response**](_birddogptzsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogsil2codec_get

> models::Birddogsil2codecGet200Response birddogsil2codec_get(protocol, selected_mode, quant_factor_i, quant_factor_p, gop_size, bitrate)
Retrieve Silicon2 Codec Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | Option<**String**> | RTSP, SRT, HX, RTMP, DISABLE |  |
**selected_mode** | Option<**String**> | Custom, low, med, high, ultra |  |
**quant_factor_i** | Option<**String**> | Range (18 to 47) |  |
**quant_factor_p** | Option<**String**> | Range (18 to 47) |  |
**gop_size** | Option<**String**> | Range (1 to 59) |  |
**bitrate** | Option<**String**> | Range (1 to 80) |  |

### Return type

[**models::Birddogsil2codecGet200Response**](_birddogsil2codec_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogsil2codec_post

> models::Birddogsil2codecGet200Response birddogsil2codec_post(birddogsil2codec_post_request, protocol, bitrate_control, mode_sel, quant_factor_i, quant_factor_p, gop_size, bitrate)
Set Silicon2 Codec Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogsil2codec_post_request** | [**Birddogsil2codecPostRequest**](Birddogsil2codecPostRequest.md) |  | [required] |
**protocol** | Option<**String**> | SRT, RTSP, RTMP, HX, DISABLE |  |
**bitrate_control** | Option<**String**> | vbr, cbr |  |
**mode_sel** | Option<**String**> | Custom, low, med, high, ultra |  |
**quant_factor_i** | Option<**String**> | Range (18 to 47) |  |
**quant_factor_p** | Option<**String**> | Range (18 to 47) |  |
**gop_size** | Option<**String**> | Range (1 to 59) |  |
**bitrate** | Option<**String**> | Range (1 to 80) |  |

### Return type

[**models::Birddogsil2codecGet200Response**](_birddogsil2codec_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogsil2enc_get

> models::Birddogsil2encGet200Response birddogsil2enc_get(streaming_protocol, port, stream_name, auth_enable, user_name, password, ip_address, mode, latency, encryption, passphrase, pbkeylen, streamid, server_selection, stream_key_local, stream_key_remote, server, hai_vision_player_support)
Retrieve Silicon2 Encode Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**streaming_protocol** | Option<**String**> | RTSP, SRT, HX, RTMP, DISABLE |  |
**port** | Option<**String**> | RTSP Port or Port For SRT |  |
**stream_name** | Option<**String**> | RTSP Stream Name |  |
**auth_enable** | Option<**String**> | RTSP Encryption or true / false |  |
**user_name** | Option<**String**> | Minimum 4 Characters |  |
**password** | Option<**String**> | Minimum 4 Characters |  |
**ip_address** | Option<**String**> | SRT IP Address |  |
**mode** | Option<**String**> | caller, listener, rendezvous |  |
**latency** | Option<**String**> | Range (80 to 8000) |  |
**encryption** | Option<**String**> | true / false |  |
**passphrase** | Option<**String**> | Text Length (10 to 79 Characters) |  |
**pbkeylen** | Option<**String**> | 16, 24, 32 |  |
**streamid** | Option<**String**> | up to 512 |  |
**server_selection** | Option<**String**> | RTMP - local, remote |  |
**stream_key_local** | Option<**String**> | Text Length (10 to 79 Characters) |  |
**stream_key_remote** | Option<**String**> | Text Length (10 to 79 Characters) |  |
**server** | Option<**String**> | Remote Server URL |  |
**hai_vision_player_support** | Option<**String**> | true / false |  |

### Return type

[**models::Birddogsil2encGet200Response**](_birddogsil2enc_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogsil2enc_post

> models::Birddogsil2encGet200Response birddogsil2enc_post(birddogsil2enc_get200_response, streaming_protocol, port, stream_name, auth_enable, user_name, password, ip_address, mode, latency, encryption, passphrase, pbkeylen, streamid, server_selection, stream_key_local, stream_key_remote, server, hai_vision_player_support)
Set Silicon2 Encode Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogsil2enc_get200_response** | [**Birddogsil2encGet200Response**](Birddogsil2encGet200Response.md) |  | [required] |
**streaming_protocol** | Option<**String**> | RTSP, SRT, HX, RTMP, DISABLE |  |
**port** | Option<**String**> | RTSP Port or Port For SRT |  |
**stream_name** | Option<**String**> | RTSP Stream Name |  |
**auth_enable** | Option<**String**> | RTSP Encryption or true / false |  |
**user_name** | Option<**String**> | Minimum 4 Characters |  |
**password** | Option<**String**> | Minimum 4 Characters |  |
**ip_address** | Option<**String**> | SRT IP Address |  |
**mode** | Option<**String**> | caller, listener, rendezvous |  |
**latency** | Option<**String**> | Range (80 to 8000) |  |
**encryption** | Option<**String**> | true / false |  |
**passphrase** | Option<**String**> | Text Length (10 to 79 Characters) |  |
**pbkeylen** | Option<**String**> | 16, 24, 32 |  |
**streamid** | Option<**String**> | up to 512 |  |
**server_selection** | Option<**String**> | RTMP - local, remote |  |
**stream_key_local** | Option<**String**> | Text Length (10 to 79 Characters) |  |
**stream_key_remote** | Option<**String**> | Text Length (10 to 79 Characters) |  |
**server** | Option<**String**> | Remote Server URL |  |
**hai_vision_player_support** | Option<**String**> | true / false |  |

### Return type

[**models::Birddogsil2encGet200Response**](_birddogsil2enc_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogwbsetup_get

> models::BirddogwbsetupGet200Response birddogwbsetup_get(bg, br, blue_gain, color_temp, gb, gr, level, matrix, offset, phase, rb, rg, red_gain, select, speed, wb_mode)
Retrieve White Balance Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bg** | Option<**String**> | Range (-99 to 99) |  |
**br** | Option<**String**> | Range (-99 to 99) |  |
**blue_gain** | Option<**String**> | Range (0 to 255) |  |
**color_temp** | Option<**String**> | Range (2800 to 6500) |  |
**gb** | Option<**String**> | Range (-99 to 99) |  |
**gr** | Option<**String**> | Range (-99 to 99) |  |
**level** | Option<**String**> | Range (0 to 14) |  |
**matrix** | Option<**String**> | On, Off |  |
**offset** | Option<**String**> | Range (0 to 14) |  |
**phase** | Option<**String**> | Range (0 to 14) |  |
**rb** | Option<**String**> | Range (-99 to 99) |  |
**rg** | Option<**String**> | Range (-99 to 99) |  |
**red_gain** | Option<**String**> | Range (0 to 255) |  |
**select** | Option<**String**> | FL LIGHT, STD, OFF, HIGH SAT |  |
**speed** | Option<**String**> | Range (1 to 5) |  |
**wb_mode** | Option<**String**> | AUTO, INDOOR, OUTDOOR, OUTDOOR-AUTO, ONEPUSH, ATW, MANUAL, SVL-AUTO, SVL, SVL-OUTDOOR-AUTO |  |

### Return type

[**models::BirddogwbsetupGet200Response**](_birddogwbsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## birddogwbsetup_post

> models::BirddogwbsetupGet200Response birddogwbsetup_post(birddogwbsetup_get200_response, bg, br, blue_gain, color_temp, gb, gr, level, matrix, offset, phase, rb, rg, red_gain, select, speed, wb_mode)
Set White Balance Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**birddogwbsetup_get200_response** | [**BirddogwbsetupGet200Response**](BirddogwbsetupGet200Response.md) |  | [required] |
**bg** | Option<**String**> | Range (-99 to 99) |  |
**br** | Option<**String**> | Range (-99 to 99) |  |
**blue_gain** | Option<**String**> | Range (0 to 255) |  |
**color_temp** | Option<**String**> | Range (2800 to 6500) |  |
**gb** | Option<**String**> | Range (-99 to 99) |  |
**gr** | Option<**String**> | Range (-99 to 99) |  |
**level** | Option<**String**> | Range (0 to 14) |  |
**matrix** | Option<**String**> | On, Off |  |
**offset** | Option<**String**> | Range (0 to 14) |  |
**phase** | Option<**String**> | Range (0 to 14) |  |
**rb** | Option<**String**> | Range (-99 to 99) |  |
**rg** | Option<**String**> | Range (-99 to 99) |  |
**red_gain** | Option<**String**> | Range (0 to 255) |  |
**select** | Option<**String**> | FL LIGHT, STD, OFF, HIGH SAT |  |
**speed** | Option<**String**> | Range (1 to 5) |  |
**wb_mode** | Option<**String**> | AUTO, INDOOR, OUTDOOR, OUTDOOR-AUTO, ONEPUSH, ATW, MANUAL, SVL-AUTO, SVL, SVL-OUTDOOR-AUTO |  |

### Return type

[**models::BirddogwbsetupGet200Response**](_birddogwbsetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## capture_get

> String capture_get(ch_num, status)
Capture screensaver frame for Encode/Decode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**status** | Option<**String**> | Encode, Decode |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_to_get

> models::ConnectToGet200Response connect_to_get(ch_num, sourcename, status)
Retrieve connected NDI Source info (sourceName)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**sourcename** | Option<**String**> | None |  |
**status** | Option<**String**> | Status can be 'Encode' or 'Decode' |  |

### Return type

[**models::ConnectToGet200Response**](_connectTo_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_to_post

> models::ConnectToGet200Response connect_to_post(connect_to_get200_response, ch_num, sourcename, status)
Connects to NDI Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connect_to_get200_response** | [**ConnectToGet200Response**](ConnectToGet200Response.md) |  | [required] |
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**sourcename** | Option<**String**> | None |  |
**status** | Option<**String**> | Status can be 'Encode' or 'Decode' |  |

### Return type

[**models::ConnectToGet200Response**](_connectTo_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_transport_get

> models::DecodeTransportGet200Response decode_transport_get(rxpm)
Retrieve NDI network settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rxpm** | Option<**String**> | Multicast, TCP, Multi-TCP, UDP |  |

### Return type

[**models::DecodeTransportGet200Response**](_decodeTransport_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_transport_post

> models::DecodeTransportGet200Response decode_transport_post(decode_transport_get200_response, rxpm)
Set NDI network settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decode_transport_get200_response** | [**DecodeTransportGet200Response**](DecodeTransportGet200Response.md) |  | [required] |
**rxpm** | Option<**String**> | Multicast, TCP, Multi-TCP, UDP |  |

### Return type

[**models::DecodeTransportGet200Response**](_decodeTransport_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decodesetup_get

> models::DecodesetupGet200Response decodesetup_get(ch_num, color_space, tally_mode, screen_saver_mode, ndi_audio)
Retrieve Decode Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**color_space** | Option<**String**> | RGB, YUV |  |
**tally_mode** | Option<**String**> | TallyOn, TallyOff, VideoMode |  |
**screen_saver_mode** | Option<**String**> | BirdDogSS, BlackSS, CaptureSS |  |
**ndi_audio** | Option<**String**> | NDIAudioEn, NDIAudioDis |  |

### Return type

[**models::DecodesetupGet200Response**](_decodesetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decodesetup_post

> models::DecodesetupGet200Response decodesetup_post(decodesetup_get200_response, ch_num, color_space, tally_mode, screen_saver_mode, ndi_audio)
Set Decode Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decodesetup_get200_response** | [**DecodesetupGet200Response**](DecodesetupGet200Response.md) |  | [required] |
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**color_space** | Option<**String**> | RGB, YUV |  |
**tally_mode** | Option<**String**> | TallyOn, TallyOff, VideoMode |  |
**screen_saver_mode** | Option<**String**> | BirdDogSS, BlackSS, CaptureSS |  |
**ndi_audio** | Option<**String**> | NDIAudioEn, NDIAudioDis |  |

### Return type

[**models::DecodesetupGet200Response**](_decodesetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decodestatus_get

> models::DecodestatusGet200Response decodestatus_get(ch_num)
Retrieve connected NDI Source Status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |

### Return type

[**models::DecodestatusGet200Response**](_decodestatus_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_transport_get

> models::EncodeTransportGet200Response encode_transport_get(txpm, txnetprefix, txnetmask, txmcttl)
Retrieve current NDI Network Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txpm** | Option<**String**> | Multicast, TCP, Multi-TCP, UDP |  |
**txnetprefix** | Option<**String**> | xxx.xxx.xxx.xxx |  |
**txnetmask** | Option<**String**> | xxx.xxx.xxx.xxx |  |
**txmcttl** | Option<**String**> | 1 |  |

### Return type

[**models::EncodeTransportGet200Response**](_encodeTransport_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_transport_post

> models::EncodeTransportGet200Response encode_transport_post(encode_transport_get200_response, txpm, txnetprefix, txnetmask, txmcttl)
Set NDI Network Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**encode_transport_get200_response** | [**EncodeTransportGet200Response**](EncodeTransportGet200Response.md) |  | [required] |
**txpm** | Option<**String**> | Multicast, TCP, Multi-TCP, UDP |  |
**txnetprefix** | Option<**String**> | xxx.xxx.xxx.xxx |  |
**txnetmask** | Option<**String**> | xxx.xxx.xxx.xxx |  |
**txmcttl** | Option<**String**> | 1 |  |

### Return type

[**models::EncodeTransportGet200Response**](_encodeTransport_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encodesetup_get

> models::EncodesetupGet200Response encodesetup_get(ch_num, video_format, video_sample_rate, color_bit_depth, stream_name, ndi_audio, screen_saver_mode, bandwidth_mode, bandwidth_select, loop_tally, tally_mode, video_csc, ndi_group, ndi_group_name)
Retrieve Encode Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**video_format** | Option<**String**> | VideoFormat |  |
**video_sample_rate** | Option<**String**> | 420, 422, 444 |  |
**color_bit_depth** | Option<**String**> | 8Bit,10Bit,12Bit |  |
**stream_name** | Option<**String**> | Name of Stream |  |
**ndi_audio** | Option<**String**> | NDIAudioMain, NDIAudioAnalog, NDIAudioMute |  |
**screen_saver_mode** | Option<**String**> | BirdDogSS, BlackSS, CaptureSS |  |
**bandwidth_mode** | Option<**String**> | Manual, NDIManaged |  |
**bandwidth_select** | Option<**String**> | Range (60 to 360) |  |
**loop_tally** | Option<**String**> | LoopTallyEn, LoopTallyDis |  |
**tally_mode** | Option<**String**> | TallyOn, TallyOff, VideoMode |  |
**video_csc** | Option<**String**> | NoCSC, RGB, YVU, YUV444, YVU444 |  |
**ndi_group** | Option<**String**> | NDIGroupEn, NDIGroupDis |  |
**ndi_group_name** | Option<**String**> | BirdDog |  |

### Return type

[**models::EncodesetupGet200Response**](_encodesetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encodesetup_post

> models::EncodesetupGet200Response encodesetup_post(encodesetup_get200_response, ch_num, video_format, video_sample_rate, color_bit_depth, stream_name, ndi_audio, screen_saver_mode, bandwidth_mode, bandwidth_select, loop_tally, tally_mode, video_csc, ndi_group, ndi_group_name)
Set Encode Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**encodesetup_get200_response** | [**EncodesetupGet200Response**](EncodesetupGet200Response.md) |  | [required] |
**ch_num** | Option<**String**> | Channel No. Range 1 to 4 |  |
**video_format** | Option<**String**> | VideoFormat |  |
**video_sample_rate** | Option<**String**> | 420, 422, 444 |  |
**color_bit_depth** | Option<**String**> | 8Bit,10Bit,12Bit |  |
**stream_name** | Option<**String**> | Name of Stream |  |
**ndi_audio** | Option<**String**> | NDIAudioMain, NDIAudioAnalog, NDIAudioMute |  |
**screen_saver_mode** | Option<**String**> | BirdDogSS, BlackSS, CaptureSS |  |
**bandwidth_mode** | Option<**String**> | Manual, NDIManaged |  |
**bandwidth_select** | Option<**String**> | Range (60 to 360) |  |
**loop_tally** | Option<**String**> | LoopTallyEn, LoopTallyDis |  |
**tally_mode** | Option<**String**> | TallyOn, TallyOff, VideoMode |  |
**video_csc** | Option<**String**> | NoCSC, RGB, YVU, YUV444, YVU444 |  |
**ndi_group** | Option<**String**> | NDIGroupEn, NDIGroupDis |  |
**ndi_group_name** | Option<**String**> | BirdDog |  |

### Return type

[**models::EncodesetupGet200Response**](_encodesetup_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hostname_get

> String hostname_get()
Retrieve device hostname

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_get

> models::ListGet200Response list_get()
Retrieve List of active NDI Sources on the Network

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListGet200Response**](_List_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_di_dis_server_get

> models::NdiDisServerGet200Response n_di_dis_server_get(ndi_dis_serv, ndi_dis_serv_ip)
Retrieve NDI Discovery server info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ndi_dis_serv** | Option<**String**> | NDIDisServEn/NDIDisServDis |  |
**ndi_dis_serv_ip** | Option<**String**> | xxx.xxx.xxx.xxx |  |

### Return type

[**models::NdiDisServerGet200Response**](_NDIDisServer_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_di_dis_server_post

> models::NdiDisServerGet200Response n_di_dis_server_post(ndi_dis_server_get200_response, ndi_dis_serv, ndi_dis_serv_ip)
Set NDI Discovery server info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ndi_dis_server_get200_response** | [**NdiDisServerGet200Response**](NdiDisServerGet200Response.md) |  | [required] |
**ndi_dis_serv** | Option<**String**> | NDIDisServEn/NDIDisServDis |  |
**ndi_dis_serv_ip** | Option<**String**> | xxx.xxx.xxx.xxx |  |

### Return type

[**models::NdiDisServerGet200Response**](_NDIDisServer_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_di_grp_name_get

> String n_di_grp_name_get()
Retrieve GroupName

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_di_grp_name_post

> String n_di_grp_name_post(body)
Set GroupName

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_di_off_sn_src_get

> String n_di_off_sn_src_get()
Retrieve Off Subnet IP List

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_di_off_sn_src_post

> String n_di_off_sn_src_post(body)
Set Off Subnet IP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operationmode_get

> String operationmode_get(mode)
Retrieve current operation mode of your BirdDog device (encode/decode)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mode** | Option<**String**> | Device operation mode 'encode' 'decode' |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operationmode_post

> String operationmode_post(body, mode)
Set base operation modes of your BirdDog device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**mode** | Option<**String**> | Device operation mode 'encode' 'decode' |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_get

> serde_json::Value reboot_get()
Reboot BirdDog device

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_post

> serde_json::Value reboot_post()
Reboot BirdDog device

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recall_post

> models::RecallPostRequest recall_post(recall_post_request, preset)
Recall Preset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recall_post_request** | [**RecallPostRequest**](RecallPostRequest.md) |  | [required] |
**preset** | Option<**String**> | Preset-1 (Range from 1 to 9) |  |

### Return type

[**models::RecallPostRequest**](_recall_post_request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_get

> serde_json::Value refresh_get()
Refresh NDI Source List

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_post

> serde_json::Value refresh_post()
Refresh NDI Source List

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_get

> serde_json::Value reset_get()
Reset NDI Source List

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_post

> serde_json::Value reset_post()
Reset NDI Source List

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_get

> serde_json::Value restart_get()
Restart video subsystem on device

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_post

> serde_json::Value restart_post()
Restart video subsystem on device

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_post

> models::RecallPostRequest save_post(recall_post_request, preset)
Save Preset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recall_post_request** | [**RecallPostRequest**](RecallPostRequest.md) |  | [required] |
**preset** | Option<**String**> | Preset-1 (Range from 1 to 9) |  |

### Return type

[**models::RecallPostRequest**](_recall_post_request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version_get

> String version_get()
Hardware version number query

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videooutputinterface_get

> String videooutputinterface_get(videooutput)
Retrieve current video output interface mode of your BirdDog device (sdi/hdmi) or (LowLatency/NormalMode)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**videooutput** | Option<**String**> | Device operation mode 'sdi' 'hdmi' 'LowLatency' 'NormalMode' |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videooutputinterface_post

> String videooutputinterface_post(body, videooutput)
Set base video output interface mode of your BirdDog device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** |  | [required] |
**videooutput** | Option<**String**> | Device video output mode 'sdi' 'hdmi' 'LowLatency' 'NormalMode' |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

