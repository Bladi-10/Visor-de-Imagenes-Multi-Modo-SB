use std::path::PathBuf;
use crate::app::model::ViewMode;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppMsg {
    // Control de Ventana y Modos Visuales
    ToggleFullscreen,
    ToggleCleanUI,
    MinimizeWindow,
    MaximizeWindow,
    CloseApp,
    ShowUserManual,

    // Navegacion e Interaccion
    NextImage,
    PreviousImage,
    SelectActiveImage(usize),
    SetViewMode(ViewMode),

    // Zoom
    ZoomIn,
    ZoomOut,
    ZoomReset,
    ZoomFit,

    // Gestion de Archivos y Papelera
    OpenSingleFile,
    OpenFolder,
    SingleFileSelected(Option<PathBuf>),
    FolderSelected(Option<PathBuf>),
    TrashActiveImage,
    PermanentlyDeleteActiveImage,
    ConfirmPermanentDelete,
    UndoDelete,

    // Visualizacion
    ToggleSidebar,
}
