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
            if let Some(scroll) = child.downcast_ref::<gtk::ScrolledWindow>() {
                if let Some(picture) = scroll.child().and_then(|c| c.downcast::<gtk::Picture>().ok()) {
                    picture.set_paintable(None::<&gtk::gdk::Texture>);
                }
            }
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
                let scroll = gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .hexpand(true)
                    .vexpand(true)
                    .build();

                let drag = gtk::GestureDrag::new();
                let scroll_clone = scroll.clone();
                let start_h = std::rc::Rc::new(std::cell::Cell::new(0.0));
                let start_v = std::rc::Rc::new(std::cell::Cell::new(0.0));

                let start_h_begin = start_h.clone();
                let start_v_begin = start_v.clone();
                let scroll_begin = scroll.clone();

                drag.connect_drag_begin(move |_, _, _| {
                    let h = scroll_begin.hadjustment();
                    let v = scroll_begin.vadjustment();
                    start_h_begin.set(h.value());
                    start_v_begin.set(v.value());
                });

                drag.connect_drag_update(move |_, offset_x, offset_y| {
                    let h = scroll_clone.hadjustment();
                    let v = scroll_clone.vadjustment();
                    h.set_value(start_h.get() - offset_x);
                    v.set_value(start_v.get() - offset_y);
                });

                scroll.add_controller(drag);

                if let Some(texture) = &item.texture {
                    let picture = gtk::Picture::for_paintable(texture);
                    picture.set_halign(gtk::Align::Center);
                    picture.set_valign(gtk::Align::Center);
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
                    let lbl_err = gtk::Label::new(Some("Cargando vista previa de la imagen..."));
                    scroll.set_child(Some(&lbl_err));
                }

                self.row_box.append(&scroll);
            }
        }
    }
}
