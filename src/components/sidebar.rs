use gtk::prelude::*;
use crate::utils::metadata::{format_size, ImageMetadata};

pub struct SidebarComponent {
    pub container: gtk::Box,
    lbl_file_name: gtk::Label,
    lbl_file_path: gtk::Label,
    lbl_dimensions: gtk::Label,
    lbl_size: gtk::Label,
    lbl_mime: gtk::Label,
    lbl_camera: gtk::Label,
    lbl_iso: gtk::Label,
    lbl_aperture: gtk::Label,
}

impl SidebarComponent {
    pub fn new() -> Self {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .width_request(260)
            .build();

        let header_lbl = gtk::Label::builder()
            .label("<b>Detalles de la Imagen</b>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .build();

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

        let lbl_file_name = Self::create_detail_label("Nombre: -");
        let lbl_file_path = Self::create_detail_label("Ruta: -");
        let lbl_dimensions = Self::create_detail_label("Dimensiones: -");
        let lbl_size = Self::create_detail_label("Tamaño: -");
        let lbl_mime = Self::create_detail_label("Tipo: -");

        let exif_header = gtk::Label::builder()
            .label("<b>Metadatos EXIF</b>")
            .use_markup(true)
            .halign(gtk::Align::Start)
            .margin_top(12)
            .build();

        let lbl_camera = Self::create_detail_label("Cámara: N/A");
        let lbl_iso = Self::create_detail_label("ISO: N/A");
        let lbl_aperture = Self::create_detail_label("Apertura: N/A");

        container.append(&header_lbl);
        container.append(&separator);
        container.append(&lbl_file_name);
        container.append(&lbl_file_path);
        container.append(&lbl_dimensions);
        container.append(&lbl_size);
        container.append(&lbl_mime);
        container.append(&exif_header);
        container.append(&lbl_camera);
        container.append(&lbl_iso);
        container.append(&lbl_aperture);

        Self {
            container,
            lbl_file_name,
            lbl_file_path,
            lbl_dimensions,
            lbl_size,
            lbl_mime,
            lbl_camera,
            lbl_iso,
            lbl_aperture,
        }
    }

    fn create_detail_label(text: &str) -> gtk::Label {
        gtk::Label::builder()
            .label(text)
            .halign(gtk::Align::Start)
            .wrap(true)
            .build()
    }

    pub fn update_metadata(&self, meta_opt: Option<&ImageMetadata>) {
        if let Some(meta) = meta_opt {
            self.lbl_file_name.set_label(&format!("Nombre: {}", meta.file_name));
            self.lbl_file_path.set_label(&format!("Ruta: {}", meta.file_path));
            self.lbl_dimensions.set_label(&format!("Dimensiones: {}x{}", meta.dimensions.0, meta.dimensions.1));
            self.lbl_size.set_label(&format!("Tamaño: {}", format_size(meta.size_bytes)));
            self.lbl_mime.set_label(&format!("Tipo: {}", meta.mime_type));

            self.lbl_camera.set_label(&format!(
                "Cámara: {}",
                meta.camera_model.as_deref().unwrap_or("N/A")
            ));
            self.lbl_iso.set_label(&format!(
                "ISO: {}",
                meta.iso.map(|v| v.to_string()).as_deref().unwrap_or("N/A")
            ));
            self.lbl_aperture.set_label(&format!(
                "Apertura: {}",
                meta.f_number.map(|v| format!("f/{:.1}", v)).as_deref().unwrap_or("N/A")
            ));
        } else {
            self.lbl_file_name.set_label("Nombre: -");
            self.lbl_file_path.set_label("Ruta: -");
            self.lbl_dimensions.set_label("Dimensiones: -");
            self.lbl_size.set_label("Tamaño: -");
            self.lbl_mime.set_label("Tipo: -");
            self.lbl_camera.set_label("Cámara: N/A");
            self.lbl_iso.set_label("ISO: N/A");
            self.lbl_aperture.set_label("Apertura: N/A");
        }
    }
}
