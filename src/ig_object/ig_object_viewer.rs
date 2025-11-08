use egui_ltreeview::TreeViewBuilder;
use ig_library::core::ig_objects::igObject;
use crate::tabs::tfb_editor::TfbToolEditor;

/// Responsible for making an igObject viewable and/or editable. Full control of the UI is present here, meaning you can do anything you would like. However, if you want bigger changes (e.g., a level editor/viewer), you will need to make your own editor instead of igObjectViewer
#[allow(non_camel_case_types)]
pub trait igObjectInterface: Send + Sync {
    /// This is the first function run to determine if this viewer is the right fit for the object. This method should be very efficient as it will be run once per frame
    fn should_display(&self, object: igObject) -> bool;

    /// This viewer has been chosen to render the viewer for this object. In this method, you are responsible for writing the code to handle what the editor/viewer looks like. Ideally make sure all IDs you use here are unique, as many objects may be open and TFB tends to share names around a lot. Doing so will make checking for interactions with the UI later on much easier.
    fn display(&self, builder: TreeViewBuilder<u32>, editor: &mut TfbToolEditor, object: igObject);

    /// Callback called when a node is selected. Allows you to turn this into an editor.
    fn node_selected(&self, editor: &mut TfbToolEditor, node: u32);
}
