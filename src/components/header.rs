use gtk::prelude::*;
use relm4::Sender;
use crate::app::model::{AppModel, ViewMode};
use crate::app::msg::AppMsg;

pub struct HeaderComponent {
    pub header_bar: adw::HeaderBar,
    btn_mode_1: gtk::ToggleButton,
    btn_mode_2: gtk::ToggleButton,
    btn_mode_3: gtk::ToggleButton,
    btn_mode_4: gtk::ToggleButton,
    lbl_status: gtk::Label,
}

impl HeaderComponent {
    pub fn new(sender: Sender<AppMsg>) -> Self {
        let header_bar = adw::HeaderBar::new();

        // Botones de la izquierda
        let box_left = gtk::Box::new(gtk::Orientation::Horizontal, 4);

        let btn_info = gtk::Button::from_icon_name("info-symbolic");
        btn_info.set_tooltip_text(Some("Manual de Usuario (Info)"));
        let sender_info = sender.clone();
        btn_info.connect_clicked(move |_| {
            sender_info.send(AppMsg::ShowUserManual).unwrap();
        });

        let btn_fullscreen = gtk::Button::from_icon_name("view-fullscreen-symbolic");
        btn_fullscreen.set_tooltip_text(Some("Pantalla Completa (F11)"));
        let sender_fs = sender.clone();
        btn_fullscreen.connect_clicked(move |_| {
            sender_fs.send(AppMsg::ToggleFullscreen).unwrap();
        });

        let btn_open_file = gtk::Button::from_icon_name("document-open-symbolic");
        btn_open_file.set_tooltip_text(Some("Abrir Archivo de Imagen"));
        let sender_file = sender.clone();
        btn_open_file.connect_clicked(move |_| {
            sender_file.send(AppMsg::OpenSingleFile).unwrap();
        });

        let btn_open_folder = gtk::Button::from_icon_name("folder-open-symbolic");
        btn_open_folder.set_tooltip_text(Some("Abrir Carpeta de Imágenes"));
        let sender_folder = sender.clone();
        btn_open_folder.connect_clicked(move |_| {
            sender_folder.send(AppMsg::OpenFolder).unwrap();
        });

        box_left.append(&btn_info);
        box_left.append(&btn_fullscreen);
        box_left.append(&btn_open_file);
        box_left.append(&btn_open_folder);
        header_bar.pack_start(&box_left);

        // Selector de Modos (Centro)
        let box_center = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        box_center.add_css_class("linked");

        let btn_mode_1 = gtk::ToggleButton::with_label("1");
        let btn_mode_2 = gtk::ToggleButton::with_label("2");
        btn_mode_2.set_group(Some(&btn_mode_1));
        let btn_mode_3 = gtk::ToggleButton::with_label("3");
        btn_mode_3.set_group(Some(&btn_mode_1));
        let btn_mode_4 = gtk::ToggleButton::with_label("4");
        btn_mode_4.set_group(Some(&btn_mode_1));

        btn_mode_1.set_active(true);

        let sender_m1 = sender.clone();
        btn_mode_1.connect_clicked(move |_| {
            sender_m1.send(AppMsg::SetViewMode(ViewMode::Single)).unwrap();
        });
        let sender_m2 = sender.clone();
        btn_mode_2.connect_clicked(move |_| {
            sender_m2.send(AppMsg::SetViewMode(ViewMode::Dual)).unwrap();
        });
        let sender_m3 = sender.clone();
        btn_mode_3.connect_clicked(move |_| {
            sender_m3.send(AppMsg::SetViewMode(ViewMode::Triple)).unwrap();
        });
        let sender_m4 = sender.clone();
        btn_mode_4.connect_clicked(move |_| {
            sender_m4.send(AppMsg::SetViewMode(ViewMode::Quad)).unwrap();
        });

        box_center.append(&btn_mode_1);
        box_center.append(&btn_mode_2);
        box_center.append(&btn_mode_3);
        box_center.append(&btn_mode_4);

        let lbl_status = gtk::Label::new(Some("Visor de Imágenes"));
        let title_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        title_box.append(&lbl_status);
        title_box.append(&box_center);

        header_bar.set_title_widget(Some(&title_box));

        // Controles de la Derecha
        let box_right = gtk::Box::new(gtk::Orientation::Horizontal, 4);

        let btn_zoom_out = gtk::Button::from_icon_name("zoom-out-symbolic");
        let sender_zo = sender.clone();
        btn_zoom_out.connect_clicked(move |_| {
            sender_zo.send(AppMsg::ZoomOut).unwrap();
        });

        let btn_zoom_in = gtk::Button::from_icon_name("zoom-in-symbolic");
        let sender_zi = sender.clone();
        btn_zoom_in.connect_clicked(move |_| {
            sender_zi.send(AppMsg::ZoomIn).unwrap();
        });

        let btn_zoom_1 = gtk::Button::with_label("1:1");
        let sender_z1 = sender.clone();
        btn_zoom_1.connect_clicked(move |_| {
            sender_z1.send(AppMsg::ZoomReset).unwrap();
        });

        let btn_zoom_fit = gtk::Button::with_label("Fit");
        let sender_zf = sender.clone();
        btn_zoom_fit.connect_clicked(move |_| {
            sender_zf.send(AppMsg::ZoomFit).unwrap();
        });

        let btn_sidebar = gtk::Button::from_icon_name("sidebar-show-symbolic");
        btn_sidebar.set_tooltip_text(Some("Alternar Panel de Detalles (Ctrl+E)"));
        let sender_side = sender.clone();
        btn_sidebar.connect_clicked(move |_| {
            sender_side.send(AppMsg::ToggleSidebar).unwrap();
        });

        let btn_minimize = gtk::Button::from_icon_name("window-minimize-symbolic");
        btn_minimize.set_tooltip_text(Some("Minimizar Ventana"));
        let sender_min = sender.clone();
        btn_minimize.connect_clicked(move |_| {
            sender_min.send(AppMsg::MinimizeWindow).unwrap();
        });

        let btn_maximize = gtk::Button::from_icon_name("window-maximize-symbolic");
        btn_maximize.set_tooltip_text(Some("Maximizar Ventana"));
        let sender_max = sender.clone();
        btn_maximize.connect_clicked(move |_| {
            sender_max.send(AppMsg::MaximizeWindow).unwrap();
        });

        box_right.append(&btn_zoom_out);
        box_right.append(&btn_zoom_in);
        box_right.append(&btn_zoom_1);
        box_right.append(&btn_zoom_fit);
        box_right.append(&btn_sidebar);
        box_right.append(&btn_minimize);
        box_right.append(&btn_maximize);

        header_bar.pack_end(&box_right);

        Self {
            header_bar,
            btn_mode_1,
            btn_mode_2,
            btn_mode_3,
            btn_mode_4,
            lbl_status,
        }
    }

    pub fn update(&self, model: &AppModel) {
        match model.view_mode {
            ViewMode::Single => self.btn_mode_1.set_active(true),
            ViewMode::Dual => self.btn_mode_2.set_active(true),
            ViewMode::Triple => self.btn_mode_3.set_active(true),
            ViewMode::Quad => self.btn_mode_4.set_active(true),
        }

        if model.images.is_empty() {
            self.lbl_status.set_label("Visor de Imágenes");
        } else {
            self.lbl_status.set_label(&format!(
                "Imagen {} de {}",
                model.current_index + 1,
                model.images.len()
            ));
        }
    }
}
