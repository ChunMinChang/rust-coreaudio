extern crate coreaudio_sys;

use self::coreaudio_sys::{
    kAudioDevicePropertyAvailableNominalSampleRates,
    kAudioDevicePropertyBufferFrameSizeRange,
    kAudioDevicePropertyDeviceUID,
    kAudioDevicePropertyNominalSampleRate,
    kAudioDevicePropertyLatency,
    kAudioObjectPropertyManufacturer,
    kAudioObjectPropertyName,
    kAudioHardwarePropertyDevices,
    kAudioHardwarePropertyDefaultInputDevice,
    kAudioHardwarePropertyDefaultOutputDevice,
    kAudioDevicePropertyStreamConfiguration,
    kAudioDevicePropertyStreams,
    kAudioDevicePropertyDataSource,
    kAudioDevicePropertyDataSourceNameForIDCFString,
    kAudioObjectPropertyScopeInput,
    kAudioObjectPropertyScopeOutput,
    kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyElementMaster,
    kAudioStreamPropertyLatency,
    AudioObjectPropertyAddress,
};

// TODO: Use function to generate a property address.
// use super::Scope;
// fn get_property_address(scope: &Scope) -> AudioObjectPropertyAddress {
// }

// fn get_global_property_address(...) -> AudioObjectPropertyAddress {
// }


// TODO: Will it be different for input and output?
pub const DEVICE_UID_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDeviceUID,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

// TODO: Will it be different for input and output?
pub const DEVICE_MANUFACTURER_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioObjectPropertyManufacturer,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const DEVICE_NAME_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioObjectPropertyName,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const DEVICES_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDevices,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const DEFAULT_INPUT_DEVICE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultInputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const DEFAULT_OUTPUT_DEVICE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultOutputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_DEVICE_STREAM_CONFIGURATION_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
      mSelector: kAudioDevicePropertyStreamConfiguration,
      mScope: kAudioObjectPropertyScopeInput,
      mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_DEVICE_STREAM_CONFIGURATION_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
      mSelector: kAudioDevicePropertyStreamConfiguration,
      mScope: kAudioObjectPropertyScopeOutput,
      mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_DEVICE_STREAMS_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyStreams,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_DEVICE_STREAMS_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyStreams,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_DEVICE_SOURCE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSource,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_DEVICE_SOURCE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSource,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSourceNameForIDCFString,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_DEVICE_SOURCE_NAME_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDataSourceNameForIDCFString,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

// TODO: Will it be different from output's one?
pub const INPUT_DEVICE_SAMPLE_RATE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyNominalSampleRate,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

// TODO: Will it be different from input's one?
pub const OUTPUT_DEVICE_SAMPLE_RATE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyNominalSampleRate,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

// TODO: Will it be different from output's one?
pub const INPUT_DEVICE_AVAILABLE_SAMPLE_RATE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyAvailableNominalSampleRates,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

// TODO: Will it be different from input's one?
pub const OUTPUT_DEVICE_AVAILABLE_SAMPLE_RATE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyAvailableNominalSampleRates,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_DEVICE_LATENCY_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyLatency,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_DEVICE_LATENCY_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyLatency,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_DEVICE_BUFFER_FRAME_SIZE_RANGE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyBufferFrameSizeRange,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_DEVICE_BUFFER_FRAME_SIZE_RANGE_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyBufferFrameSizeRange,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const INPUT_STREAM_LATENCY_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioStreamPropertyLatency,
        mScope: kAudioObjectPropertyScopeInput,
        mElement: kAudioObjectPropertyElementMaster,
    };

pub const OUTPUT_STREAM_LATENCY_PROPERTY_ADDRESS: AudioObjectPropertyAddress =
    AudioObjectPropertyAddress {
        mSelector: kAudioStreamPropertyLatency,
        mScope: kAudioObjectPropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };