use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::ig_object::ig_object_viewer::igObjectInterface;

lazy_static! {
    static ref EDITORS: Mutex<Vec<u8>> = Mutex::new(vec![]);
    static ref OBJECT_INTERFACES: Mutex<Vec<Box<dyn igObjectInterface>>> = Mutex::new(vec![]);
}

pub(crate) fn get_object_interfaces() -> &'static Mutex<Vec<Box<dyn igObjectInterface>>> {
    &OBJECT_INTERFACES
}

/// Adds a new editor to the list of editors currently loaded.
extern "C" fn register_editor() {}

/// Adds a new igObject interface to the list of loaded interfaces.
extern "C" fn register_object_interface(interface: Box<dyn igObjectInterface>) {
    OBJECT_INTERFACES.lock().unwrap().push(interface);
}