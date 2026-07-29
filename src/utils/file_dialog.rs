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
    filter.add_mime_type("image/jpeg");
    filter.add_mime_type("image/png");
    filter.add_mime_type("image/webp");
    filter.add_mime_type("image/bmp");
    filter.add_mime_type("image/gif");
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

pub async fn pick_folder(parent: &gtk::Window) -> Option<PathBuf> {
    let dialog = gtk::FileChooserNative::new(
        Some("Seleccionar Carpeta de Imágenes"),
        Some(parent),
        gtk::FileChooserAction::SelectFolder,
        Some("Seleccionar"),
        Some("Cancelar"),
    );

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
