use gtk::gdk::prelude::*;
use gtk::prelude::*;
use relm4::Sender;
use crate::app::model::AppModel;
use crate::app::msg::AppMsg;

pub struct ViewportComponent {
    pub overlay: gtk::Overlay,
    pub row_box: gtk::Box,
    pub btn_prev: gtk::Button,
    pub btn_next: gtk::Button,
}

impl ViewportComponent {
    pub fn new(sender: Sender<AppMsg>) -> Self {
        let overlay = gtk::Overlay::new();

        let row_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .homogeneous(true)
            .hexpand(true)
            .vexpand(true)
            .build();

        let btn_prev = gtk::Button::builder()
            .icon_name("go-previous-symbolic")
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Center)
            .margin_start(16)
            .css_classes(vec!["osd".to_string(), "circular".to_string()])
            .build();

        let sender_prev = sender.clone();
        btn_prev.connect_clicked(move |_| {
            sender_prev.send(AppMsg::PreviousImage).unwrap();
        });

        let btn_next = gtk::Button::builder()
            .icon_name("go-next-symbolic")
            .halign(gtk::Align::End)
            .valign(gtk::Align::Center)
            .margin_end(16)
            .css_classes(vec!["osd".to_string(), "circular".to_string()])
            .build();

        let sender_next = sender.clone();
        btn_next.connect_clicked(move |_| {
            sender_next.send(AppMsg::NextImage).unwrap();
        });

        overlay.set_child(Some(&row_box));
        overlay.add_overlay(&btn_prev);
        overlay.add_overlay(&btn_next);

        Self {
            overlay,
            row_box,
            btn_prev,
            btn_next,
        }
    }

    pub fn update(&self, model: &AppModel) {
        while let Some(child) = self.row_box.first_child() {
            self.row_box.remove(&child);
        }

        if model.is_clean_ui {
            self.btn_prev.set_visible(false);
            self.btn_next.set_visible(false);
            self.row_box.add_css_class("clean-viewport");
        } else {
            self.btn_prev.set_visible(!model.images.is_empty());
            self.btn_next.set_visible(!model.images.is_empty());
            self.row_box.remove_css_class("clean-viewport");
        }

        if model.images.is_empty() {
            let empty_label = gtk::Label::builder()
                .label("<b>Sin imágenes abiertas</b>\nUse los botones de la barra superior o Ctrl+O para abrir imágenes")
                .use_markup(true)
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .build();
            self.row_box.append(&empty_label);
            return;
        }

        let visible_indices = model.visible_indices();
        for idx in visible_indices {
            if let Some(item) = model.images.get(idx) {
                let frame = gtk::Frame::builder()
                    .hexpand(true)
                    .vexpand(true)
                    .build();

                let scroll = gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .hexpand(true)
                    .vexpand(true)
                    .build();

                if let Some(texture) = &item.texture {
                    let picture = gtk::Picture::for_paintable(texture);
                    picture.set_can_shrink(model.is_fit_mode);

                    if !model.is_fit_mode {
                        let width: f64 = texture.intrinsic_width() as f64;
                        let height: f64 = texture.intrinsic_height() as f64;
                        picture.set_size_request(
                            (width * model.zoom_level) as i32,
                            (height * model.zoom_level) as i32,
                        );
                    }

                    scroll.set_child(Some(&picture));
                } else {
                    let lbl_err = gtk::Label::new(Some("No se pudo cargar la vista previa de la imagen"));
                    scroll.set_child(Some(&lbl_err));
                }

                frame.set_child(Some(&scroll));
                self.row_box.append(&frame);
            }
        }
    }
}
