use gtk::prelude::*;
use relm4::Sender;
use crate::app::msg::AppMsg;

pub fn show_close_confirmation_dialog(parent: &impl IsA<gtk::Window>, sender: Sender<AppMsg>) {
    let dialog = gtk::Window::builder()
        .title("Confirmación de Cierre - Visor de Imágenes")
        .transient_for(parent)
        .modal(true)
        .default_width(480)
        .default_height(220)
        .resizable(false)
        .build();

    let box_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let title_label = gtk::Label::builder()
        .label("<b>¿Desea guardar los cambios en las imágenes eliminadas?</b>")
        .use_markup(true)
        .halign(gtk::Align::Center)
        .build();

    let desc_label = gtk::Label::builder()
        .label(
            "Hay imágenes en la papelera de retención temporal.\n\
            • <b>Enviar a Papelera y Salir:</b> Mueve permanentemente los archivos a la papelera del SO.\n\
            • <b>Descartar Cambios y Salir:</b> Restaura todos los archivos a sus carpetas originales.",
        )
        .use_markup(true)
        .wrap(true)
        .halign(gtk::Align::Start)
        .build();

    let box_buttons = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .halign(gtk::Align::End)
        .margin_top(12)
        .build();

    let btn_commit = gtk::Button::builder()
        .label("Enviar a Papelera y Salir")
        .css_classes(vec!["destructive-action".to_string()])
        .build();

    let btn_restore = gtk::Button::builder()
        .label("Descartar Cambios y Salir")
        .build();

    let btn_cancel = gtk::Button::builder()
        .label("Cancelar")
        .build();

    let sender_commit = sender.clone();
    let dialog_commit = dialog.clone();
    btn_commit.connect_clicked(move |_| {
        sender_commit.send(AppMsg::ConfirmCommitTrashAndExit).unwrap();
        dialog_commit.close();
    });

    let sender_restore = sender.clone();
    let dialog_restore = dialog.clone();
    btn_restore.connect_clicked(move |_| {
        sender_restore.send(AppMsg::ConfirmRestoreAndExit).unwrap();
        dialog_restore.close();
    });

    let dialog_cancel = dialog.clone();
    btn_cancel.connect_clicked(move |_| {
        dialog_cancel.close();
    });

    box_buttons.append(&btn_cancel);
    box_buttons.append(&btn_restore);
    box_buttons.append(&btn_commit);

    box_container.append(&title_label);
    box_container.append(&desc_label);
    box_container.append(&box_buttons);

    dialog.set_child(Some(&box_container));
    dialog.present();
}
