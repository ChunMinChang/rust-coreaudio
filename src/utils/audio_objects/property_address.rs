extern crate coreaudio_sys;

use self::coreaudio_sys::{
    kAudioDevicePropertyAvailableNominalSampleRates, kAudioDevicePropertyBufferFrameSizeRange,
    kAudioDevicePropertyDataSource, kAudioDevicePropertyDataSourceNameForIDCFString,
    kAudioDevicePropertyDeviceUID, kAudioDevicePropertyLatency,
    kAudioDevicePropertyNominalSampleRate, kAudioDevicePropertyStreamConfiguration,
    kAudioDevicePropertyStreams, kAudioHardwarePropertyDefaultInputDevice,
    kAudioHardwarePropertyDefaultOutputDevice, kAudioHardwarePropertyDevices,
    kAudioObjectPropertyElementMaster, kAudioObjectPropertyManufacturer, kAudioObjectPropertyName,
    kAudioObjectPropertyScopeGlobal, kAudioObjectPropertyScopeInput,
    kAudioObjectPropertyScopeOutput, kAudioStreamPropertyLatency, AudioObjectPropertyAddress,
    AudioObjectPropertySelector,
};

use super::Scope;

pub enum Property {
    DefaultInputDevice,
    DefaultOutputDevice,
    DeviceBufferFrameSizeRange,
    DeviceLatency,
    DeviceManufacturer,
    DeviceName,
    DeviceRate,
    DeviceRateRange,
    DeviceSource,
    DeviceSourceName,
    DeviceStreams,
    DeviceUID,
    Devices,
    StreamConfiguration,
    StreamLatency,
}

impl From<Property> for AudioObjectPropertySelector {
    fn from(p: Property) -> Self {
        match p {
            Property::DefaultInputDevice => kAudioHardwarePropertyDefaultInputDevice,
            Property::DefaultOutputDevice => kAudioHardwarePropertyDefaultOutputDevice,
            Property::DeviceBufferFrameSizeRange => kAudioDevicePropertyBufferFrameSizeRange,
            Property::DeviceLatency => kAudioDevicePropertyLatency,
            Property::DeviceManufacturer => kAudioObjectPropertyManufacturer,
            Property::DeviceName => kAudioObjectPropertyName,
            Property::DeviceRate => kAudioDevicePropertyNominalSampleRate,
            Property::DeviceRateRange => kAudioDevicePropertyAvailableNominalSampleRates,
            Property::DeviceSource => kAudioDevicePropertyDataSource,
            Property::DeviceSourceName => kAudioDevicePropertyDataSourceNameForIDCFString,
            Property::DeviceStreams => kAudioDevicePropertyStreams,
            Property::DeviceUID => kAudioDevicePropertyDeviceUID,
            Property::Devices => kAudioHardwarePropertyDevices,
            Property::StreamConfiguration => kAudioDevicePropertyStreamConfiguration,
            Property::StreamLatency => kAudioStreamPropertyLatency,
        }
    }
}

pub fn get_scope_property_address(scope: &Scope, p: Property) -> AudioObjectPropertyAddress {
    let scope = if scope == &Scope::Input {
        kAudioObjectPropertyScopeInput
    } else {
        kAudioObjectPropertyScopeOutput
    };

    AudioObjectPropertyAddress {
        mSelector: p.into(),
        mScope: scope,
        mElement: kAudioObjectPropertyElementMaster,
    }
}

pub fn get_global_property_address(p: Property) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: p.into(),
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    }
}
