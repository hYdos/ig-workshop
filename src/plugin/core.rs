
/// Defines a visual editor you can write which can be used instead of the built-in defaults.
#[repr(C)]
pub struct EditorPlugin {
    pub display_name: String,

}

pub(crate) struct PluginData {
    pub editors: Vec<EditorPlugin>
}


/// Adds a new editor to the list of editors currently loaded.
extern "C" fn register_editor() {}