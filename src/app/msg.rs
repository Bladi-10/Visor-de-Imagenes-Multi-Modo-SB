use std::path::PathBuf;
use crate::app::model::ViewMode;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppMsg {
    // Control de Ventana y Modos Visuales
    ToggleFullscreen,
    ToggleCleanUI,
    ToggleDarkMode,
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
    SingleFileSelected(Option<PathBuf>),
    TrashActiveImage,
    UndoDelete,
    ConfirmCommitTrashAndExit,
    ConfirmRestoreAndExit,

    // Visualizacion
    ToggleSidebar,
}
