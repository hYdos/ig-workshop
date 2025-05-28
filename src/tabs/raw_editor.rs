use egui::{Ui, WidgetText};
use crate::window::{LoadedGame, WorkshopTabImpl, WorkshopTabViewer};

pub struct RawEditorTab {
    pub(crate) game: LoadedGame
}
impl WorkshopTabImpl for RawEditorTab {
    fn title(&self, _viewer: &mut WorkshopTabViewer) -> WidgetText {
        format!("{} ({})", self.game.cfg._game, self.game.cfg._platform).into()
    }

    fn ui(&mut self, ui: &mut Ui, _viewer: &mut WorkshopTabViewer) {
        ui.label(format!("Content of {:?}", self.game.cfg._game));
    }
}