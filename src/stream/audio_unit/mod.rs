extern crate coreaudio_sys as sys;

use std::fmt; // For fmt::{Debug, Formatter, Result}
use std::mem; // For mem::{uninitialized(), size_of()}
use std::os::raw::c_void;
use std::ptr; // For ptr::null_mut()

// Using PartialEq for comparison.
#[derive(PartialEq)]
pub enum Error {
    InvalidComponentID,
    NoComponentFound,
    PropertyNotWritable,
    Uninitialized,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Error::InvalidComponentID => "Using the invalid audio component.",
            Error::NoComponentFound => "No audio component matching with provided settings.",
            PropertyNotWritable => "Trying to write a non-writable property.",
            Error::Uninitialized => "Trying to run an uninitialized AudioUnit."
        };
        write!(f, "{}", printable)
    }
}

pub enum Element {
    Output = 0,
    Input  = 1,
}

pub struct AudioUnit(sys::AudioUnit);

impl AudioUnit {
    pub fn new() -> Result<AudioUnit, Error> {
        let unit = create_unit()?;
        Ok(AudioUnit(unit))
    }

    pub fn get_property_info(
        &self,
        id: sys::AudioUnitPropertyID,
        scope: sys::AudioUnitScope,
        element: Element,
    ) -> Result<(usize, bool), Error> {
        get_property_info(self.0, id, scope, element)
    }

    pub fn get_property<T>(
        &self,
        id: sys::AudioUnitPropertyID,
        scope: sys::AudioUnitScope,
        element: Element,
    ) -> Result<T, Error> {
        get_property::<T>(self.0, id, scope, element)
    }

    pub fn set_property<T>(
        &self,
        id: sys::AudioUnitPropertyID,
        scope: sys::AudioUnitScope,
        element: Element,
        data: &T,
    ) -> Result<(), Error> {
        set_property::<T>(self.0, id, scope, element, data)
    }

    pub fn initialize(&self) -> Result<(), Error> {
        init_unit(self.0)
    }

    pub fn uninitialize(&self) -> Result<(), Error> {
        uninit_unit(self.0)
    }

    pub fn start(&self) -> Result<(), Error> {
        start_unit(self.0)
    }

    pub fn stop(&self) -> Result<(), Error> {
        stop_unit(self.0)
    }
}

impl Drop for AudioUnit {
    fn drop(&mut self) {
        self.stop();
        self.uninitialize();
    }
}

fn create_unit() -> Result<sys::AudioUnit, Error> {
    let desc = sys::AudioComponentDescription {
        componentType: sys::kAudioUnitType_Output,
        componentSubType: sys::kAudioUnitSubType_DefaultOutput,
        componentManufacturer: sys::kAudioUnitManufacturer_Apple,
        componentFlags: 0,
        componentFlagsMask: 0,
    };
    let component = find_next_component(ptr::null_mut(), &desc)?;
    let instance = get_new_instance(component)?;
    Ok(instance as sys::AudioUnit)
}

fn init_unit(unit: sys::AudioUnit) -> Result<(), Error> {
    let status = unsafe { sys::AudioUnitInitialize(unit) };
    convert_to_result(status)
}

fn uninit_unit(unit: sys::AudioUnit) -> Result<(), Error> {
    let status = unsafe { sys::AudioUnitUninitialize(unit) };
    convert_to_result(status)
}

fn start_unit(unit: sys::AudioUnit) -> Result<(), Error> {
    let status = unsafe { sys::AudioOutputUnitStart(unit) };
    convert_to_result(status)
}

fn stop_unit(unit: sys::AudioUnit) -> Result<(), Error> {
    let status = unsafe { sys::AudioOutputUnitStop(unit) };
    convert_to_result(status)
}

fn get_property_info(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: Element,
) -> Result<(usize, bool), Error> {
    let mut size: usize = 0;
    let mut writable = false;
    let status = audio_unit_get_property_info(
        unit,
        id,
        scope,
        element as sys::AudioUnitElement,
        &mut size,
        &mut writable
    );
    convert_to_result(status)?;
    Ok((size, writable))
}

fn get_property<T>(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: Element,
) -> Result<T, Error> {
    let mut data: T = unsafe { mem::uninitialized() };
    let mut size = mem::size_of::<T>();
    let status = audio_unit_get_property(
        unit,
        id,
        scope,
        element as sys::AudioUnitElement,
        &mut data,
        &mut size
    );
    convert_to_result(status)?;
    Ok(data)
}

fn set_property<T>(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: Element,
    data: &T,
) -> Result<(), Error> {
    let size = mem::size_of::<T>();
    let status = audio_unit_set_property::<T>(
        unit,
        id,
        scope,
        element as sys::AudioUnitElement,
        data,
        size
    );
    convert_to_result(status)
}

fn find_next_component(
    component: sys::AudioComponent,
    description: &sys::AudioComponentDescription,
) -> Result<sys::AudioComponent, Error> {
    let component = audio_component_find_next(component, description);
    if component.is_null() {
        return Err(Error::NoComponentFound);
    } else {
        Ok(component)
    }
}

fn get_new_instance(component: sys::AudioComponent) -> Result<sys::AudioComponentInstance, Error> {
    let mut instance: sys::AudioComponentInstance = unsafe { mem::uninitialized() };
    let status = audio_component_instance_new(component, &mut instance);
    convert_to_result(status)?;
    Ok(instance)
}

fn convert_to_result(status: sys::OSStatus) -> Result<(), Error> {
    match status {
        0 => Ok(()), /* sys::noErr */
        e => Err(status_to_error(e)),
    }
}

fn status_to_error(status: sys::OSStatus) -> Error {
    match status {
        4294964296 => Error::InvalidComponentID, /* invalidComponentID: -3000 */
        sys::kAudioUnitErr_PropertyNotWritable => Error::PropertyNotWritable,
        sys::kAudioUnitErr_Uninitialized => Error::Uninitialized,
        error => panic!("Unknown error: {}", error),
    }
}

fn audio_unit_get_property_info(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: sys::AudioUnitElement,
    size: *mut usize,
    writable: *mut bool,
) -> sys::OSStatus {
    unsafe {
        sys::AudioUnitGetPropertyInfo(
            unit,
            id,
            scope,
            element,
            size as *mut sys::UInt32,
            writable as *mut sys::Boolean,
        )
    }
}

fn audio_unit_get_property<T>(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: sys::AudioUnitElement,
    data: *mut T,
    size: *mut usize,
) -> sys::OSStatus {
    unsafe {
        sys::AudioUnitGetProperty(
            unit,
            id,
            scope,
            element,
            data as *mut c_void,
            size as *mut sys::UInt32,
        )
    }
}

fn audio_unit_set_property<T>(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: sys::AudioUnitElement,
    data: *const T,
    size: usize,
) -> sys::OSStatus {
    unsafe {
        sys::AudioUnitSetProperty(
            unit,
            id,
            scope,
            element,
            data as *const c_void,
            size as sys::UInt32
        )
    }
}

fn audio_component_find_next(
    component: sys::AudioComponent,
    description: &sys::AudioComponentDescription,
) -> sys::AudioComponent {
    unsafe { sys::AudioComponentFindNext(component, description) }
}

fn audio_component_instance_new(
    component: sys::AudioComponent,
    instance: &mut sys::AudioComponentInstance,
) -> sys::OSStatus {
    unsafe { sys::AudioComponentInstanceNew(component, instance) }
}

#[cfg(test)]
mod test;
