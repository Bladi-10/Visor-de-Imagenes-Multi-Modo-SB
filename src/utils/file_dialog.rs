use gtk::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub async fn pick_single_file(parent: &gtk::Window) -> Option<PathBuf> {
    let dialog = gtk::FileChooserNative::new(
        Some("Seleccionar Imagen"),
        Some(parent),
        gtk::FileChooserAction::Open,
        Some("Abrir"),
        Some("Cancelar"),
    );

    let filter = gtk::FileFilter::new();
    filter.set_name(Some("Imágenes"));
    filter.add_pattern("*.jpg");
    filter.add_pattern("*.JPG");
    filter.add_pattern("*.jpeg");
    filter.add_pattern("*.JPEG");
    filter.add_pattern("*.png");
    filter.add_pattern("*.PNG");
    filter.add_pattern("*.webp");
    filter.add_pattern("*.WEBP");
    filter.add_pattern("*.bmp");
    filter.add_pattern("*.BMP");
    filter.add_pattern("*.gif");
    filter.add_pattern("*.GIF");
    filter.add_pattern("*.tiff");
    filter.add_pattern("*.TIFF");
    filter.add_pattern("*.ico");
    filter.add_pattern("*.ICO");
    dialog.add_filter(&filter);

    let (sender, receiver) = relm4::tokio::sync::oneshot::channel::<Option<PathBuf>>();
    let sender_cell = Arc::new(Mutex::new(Some(sender)));

    dialog.connect_response(move |dialog, response| {
        let path = if response == gtk::ResponseType::Accept {
            dialog.file().and_then(|f| f.path())
        } else {
            None
        };
        if let Ok(mut guard) = sender_cell.lock() {
            if let Some(s) = guard.take() {
                let _ = s.send(path);
            }
        }
    });

    dialog.show();
    receiver.await.ok().flatten()
}
