use adw::prelude::*;
use gtk::gdk;
use gtk::glib;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

use crate::app::model::{AppModel, ImageItem, ViewMode};
use crate::app::msg::AppMsg;
use crate::components::header::HeaderComponent;
use crate::components::manual_dialog::show_manual_dialog;
use crate::components::sidebar::SidebarComponent;
use crate::components::viewport::ViewportComponent;
use crate::utils::file_dialog::{pick_folder, pick_single_file};
use crate::utils::image_loader::ImageLoader;

#[allow(dead_code)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub main_vbox: gtk::Box,
    pub content_hbox: gtk::Box,
    pub header: HeaderComponent,
    pub viewport: ViewportComponent,
    pub sidebar: SidebarComponent,
    pub toast_overlay: adw::ToastOverlay,
}

impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = ();
    type Root = adw::ApplicationWindow;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::ApplicationWindow::builder()
            .title("Visor de Imágenes")
            .default_width(1100)
            .default_height(720)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel::new();

        let header = HeaderComponent::new(sender.input_sender().clone());
        let viewport = ViewportComponent::new(sender.input_sender().clone());
        let sidebar = SidebarComponent::new();

        // Contenedor principal vertical (HeaderBar arriba, contenido abajo)
        let main_vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .build();

        main_vbox.append(&header.header_bar);

        let content_hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(0)
            .vexpand(true)
            .hexpand(true)
            .build();

        let toast_overlay = adw::ToastOverlay::new();
        toast_overlay.set_child(Some(&viewport.overlay));
        toast_overlay.set_vexpand(true);
        toast_overlay.set_hexpand(true);

        content_hbox.append(&toast_overlay);
        content_hbox.append(&sidebar.container);

        main_vbox.append(&content_hbox);

        root.set_content(Some(&main_vbox));

        // Controlador de Teclado Global
        let key_controller = gtk::EventControllerKey::new();
        let input_sender = sender.input_sender().clone();

        key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
            let ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);

            match keyval {
                gdk::Key::_1 => {
                    input_sender.send(AppMsg::SetViewMode(ViewMode::Single)).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::_2 => {
                    input_sender.send(AppMsg::SetViewMode(ViewMode::Dual)).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::_3 => {
                    input_sender.send(AppMsg::SetViewMode(ViewMode::Triple)).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::_4 => {
                    input_sender.send(AppMsg::SetViewMode(ViewMode::Quad)).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::F11 => {
                    if ctrl {
                        input_sender.send(AppMsg::ToggleCleanUI).unwrap();
                    } else {
                        input_sender.send(AppMsg::ToggleFullscreen).unwrap();
                    }
                    glib::Propagation::Stop
                }
                gdk::Key::Left => {
                    input_sender.send(AppMsg::PreviousImage).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::Right => {
                    input_sender.send(AppMsg::NextImage).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::Delete => {
                    if ctrl {
                        input_sender.send(AppMsg::PermanentlyDeleteActiveImage).unwrap();
                    } else {
                        input_sender.send(AppMsg::TrashActiveImage).unwrap();
                    }
                    glib::Propagation::Stop
                }
                gdk::Key::z | gdk::Key::Z if ctrl => {
                    input_sender.send(AppMsg::UndoDelete).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::e | gdk::Key::E if ctrl => {
                    input_sender.send(AppMsg::ToggleSidebar).unwrap();
                    glib::Propagation::Stop
                }
                gdk::Key::Escape => {
                    input_sender.send(AppMsg::CloseApp).unwrap();
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });

        root.add_controller(key_controller);

        let widgets = AppWidgets {
            window: root,
            main_vbox,
            content_hbox,
            header,
            viewport,
            sidebar,
            toast_overlay,
        };

        widgets.viewport.update(&model);
        widgets.header.update(&model);
        widgets.sidebar.update_metadata(model.active_image().and_then(|i| i.metadata.as_ref()));
        widgets.sidebar.container.set_visible(model.is_sidebar_open);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
            }
            AppMsg::ToggleCleanUI => {
                self.is_clean_ui = !self.is_clean_ui;
                if self.is_clean_ui {
                    self.is_fullscreen = true;
                }
            }
            AppMsg::MinimizeWindow => {
                if let Some(win) = relm4::main_application().active_window() {
                    win.minimize();
                }
            }
            AppMsg::MaximizeWindow => {
                if let Some(win) = relm4::main_application().active_window() {
                    if win.is_maximized() {
                        win.unmaximize();
                    } else {
                        win.maximize();
                    }
                }
            }
            AppMsg::CloseApp => {
                if self.is_clean_ui {
                    self.is_clean_ui = false;
                    self.is_fullscreen = false;
                } else {
                    relm4::main_application().quit();
                }
            }
            AppMsg::ShowUserManual => {
                if let Some(win) = relm4::main_application().active_window() {
                    show_manual_dialog(&win);
                }
            }
            AppMsg::NextImage => {
                self.next_image();
            }
            AppMsg::PreviousImage => {
                self.prev_image();
            }
            AppMsg::SelectActiveImage(idx) => {
                if idx < self.images.len() {
                    self.current_index = idx;
                }
            }
            AppMsg::SetViewMode(mode) => {
                self.view_mode = mode;
            }
            AppMsg::ZoomIn => {
                self.is_fit_mode = false;
                self.zoom_level = (self.zoom_level * 1.25).min(5.0);
            }
            AppMsg::ZoomOut => {
                self.is_fit_mode = false;
                self.zoom_level = (self.zoom_level / 1.25).max(0.1);
            }
            AppMsg::ZoomReset => {
                self.is_fit_mode = false;
                self.zoom_level = 1.0;
            }
            AppMsg::ZoomFit => {
                self.is_fit_mode = true;
                self.zoom_level = 1.0;
            }
            AppMsg::OpenSingleFile => {
                let sender_input = sender.input_sender().clone();
                relm4::spawn_local(async move {
                    if let Some(win) = relm4::main_application().active_window() {
                        let path_opt = pick_single_file(&win).await;
                        sender_input.send(AppMsg::SingleFileSelected(path_opt)).unwrap();
                    }
                });
            }
            AppMsg::OpenFolder => {
                let sender_input = sender.input_sender().clone();
                relm4::spawn_local(async move {
                    if let Some(win) = relm4::main_application().active_window() {
                        let path_opt = pick_folder(&win).await;
                        sender_input.send(AppMsg::FolderSelected(path_opt)).unwrap();
                    }
                });
            }
            AppMsg::SingleFileSelected(Some(path)) => {
                self.images.clear();
                self.images.push(ImageItem::new(path));
                self.current_index = 0;
            }
            AppMsg::SingleFileSelected(None) => {}
            AppMsg::FolderSelected(Some(folder_path)) => {
                let paths = ImageLoader::scan_directory(&folder_path);
                self.images = paths.into_iter().map(ImageItem::new).collect();
                self.current_index = 0;
            }
            AppMsg::FolderSelected(None) => {}
            AppMsg::TrashActiveImage => {
                self.remove_current_and_trash();
            }
            AppMsg::PermanentlyDeleteActiveImage => {
                if self.images.is_empty() {
                    return;
                }
                let sender_input = sender.input_sender().clone();
                if let Some(win) = relm4::main_application().active_window() {
                    let dialog = gtk::MessageDialog::builder()
                        .transient_for(&win)
                        .modal(true)
                        .message_type(gtk::MessageType::Question)
                        .buttons(gtk::ButtonsType::OkCancel)
                        .text("¿Eliminar permanentemente este archivo?")
                        .secondary_text("Esta acción eliminará el archivo del disco y no se puede deshacer.")
                        .build();

                    dialog.connect_response(move |d, response| {
                        if response == gtk::ResponseType::Ok {
                            sender_input.send(AppMsg::ConfirmPermanentDelete).unwrap();
                        }
                        d.close();
                    });

                    dialog.present();
                }
            }
            AppMsg::ConfirmPermanentDelete => {
                self.remove_current_permanently();
            }
            AppMsg::UndoDelete => {
                self.undo_last_delete();
            }
            AppMsg::ToggleSidebar => {
                self.is_sidebar_open = !self.is_sidebar_open;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        if self.is_clean_ui {
            widgets.window.set_fullscreened(true);
            widgets.header.header_bar.set_visible(false);
            widgets.sidebar.container.set_visible(false);
        } else {
            widgets.window.set_fullscreened(self.is_fullscreen);
            widgets.header.header_bar.set_visible(true);
            widgets.sidebar.container.set_visible(self.is_sidebar_open);
        }

        widgets.header.update(self);
        widgets.viewport.update(self);
        widgets.sidebar.update_metadata(self.active_image().and_then(|i| i.metadata.as_ref()));
    }
}
