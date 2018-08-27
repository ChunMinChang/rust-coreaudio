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
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            Error::InvalidComponentID => "Using the invalid audio component.",
            Error::NoComponentFound => "No audio component matching with provided settings.",
        };
        write!(f, "{}", printable)
    }
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
        element: sys::AudioUnitElement
    ) -> Result<(usize, bool), Error> {
        get_property_info(self.0, id, scope, element)
    }

    pub fn get_property<T>(
        &self,
        id: sys::AudioUnitPropertyID,
        scope: sys::AudioUnitScope,
        element: sys::AudioUnitElement,
    ) -> Result<T, Error> {
        get_property::<T>(self.0, id, scope, element)
    }

    pub fn set_property<T>(
        &self,
        id: sys::AudioUnitPropertyID,
        scope: sys::AudioUnitScope,
        element: sys::AudioUnitElement,
        data: &T,
    ) -> Result<(), Error> {
        set_property::<T>(self.0, id, scope, element, data)
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

fn get_property_info(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: sys::AudioUnitElement,
) -> Result<(usize, bool), Error> {
    let mut size: usize = 0;
    let mut writable = false;
    let status = audio_unit_get_property_info(unit, id, scope, element, &mut size, &mut writable);
    convert_to_result(status)?;
    Ok((size, writable))
}

fn get_property<T>(
    unit: sys::AudioUnit,
    id: sys::AudioUnitPropertyID,
    scope: sys::AudioUnitScope,
    element: sys::AudioUnitElement,
) -> Result<T, Error> {
    let mut data: T = unsafe { mem::uninitialized() };
    let mut size = mem::size_of::<T>();
    let status = audio_unit_get_property(
        unit,
        id,
        scope,
        element,
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
    element: sys::AudioUnitElement,
    data: &T,
) -> Result<(), Error> {
    let size = mem::size_of::<T>();
    let status = audio_unit_set_property::<T>(unit, id, scope, element, data, size);
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
    match to_bindgen_type(status) {
        sys::noErr => Ok(()),
        e => Err(status_to_error(e)),
    }
}

fn status_to_error(status: BindgenOsstatus) -> Error {
    match status {
        4294964296 => Error::InvalidComponentID, /* invalidComponentID: -3000 */
        error => panic!("Unknown error: {}", error),
    }
}

type BindgenOsstatus = u32;
fn to_bindgen_type(status: sys::OSStatus) -> BindgenOsstatus {
    status as BindgenOsstatus
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
            size as *mut u32,
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
            size as *mut u32,
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
        sys::AudioUnitSetProperty(unit, id, scope, element, data as *const c_void, size as u32)
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
