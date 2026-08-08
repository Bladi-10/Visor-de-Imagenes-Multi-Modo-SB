use gtk::prelude::*;

pub fn show_manual_dialog(parent: &gtk::Window) {
    let dialog = gtk::Window::builder()
        .title("Manual de Usuario - Visor de Imágenes")
        .transient_for(parent)
        .modal(true)
        .default_width(520)
        .default_height(420)
        .build();

    let box_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    let title_label = gtk::Label::builder()
        .label("<b>Manual de Usuario y Atajos de Teclado</b>")
        .use_markup(true)
        .halign(gtk::Align::Center)
        .build();

    let scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .build();

    let content_label = gtk::Label::builder()
        .label(
            "<b>Modos de Vista:</b>\n\
            • Tecla 1: Vista de 1 imagen a pantalla completa\n\
            • Tecla 2: Vista de 2 imágenes en fila\n\
            • Tecla 3: Vista de 3 imágenes en fila\n\
            • Tecla 4: Vista de 4 imágenes en fila continua\n\n\
            <b>Navegación:</b>\n\
            • Flecha Izquierda / Botón &lt; : Imagen anterior\n\
            • Flecha Derecha / Botón &gt; : Imagen siguiente\n\n\
            <b>Modo Ultra Limpio:</b>\n\
            • Ctrl + R: Oculta interfaz y muestra solo las imágenes\n\n\
            <b>Gestión de Archivos:</b>\n\
            • Supr / Delete: Mueve imagen activa a la papelera del SO\n\
            • Ctrl + Z: Deshace el último borrado realizado\n\n\
            <b>Otros Atajos:</b>\n\
            • F11: Pantalla completa estándar\n\
            • Ctrl + E: Alterna panel lateral de detalles EXIF\n\
            • ESC: Cerrar diálogo / Salir del visor",
        )
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();

    scroll.set_child(Some(&content_label));

    let close_btn = gtk::Button::builder()
        .label("Cerrar")
        .halign(gtk::Align::Center)
        .build();

    let dialog_clone = dialog.clone();
    close_btn.connect_clicked(move |_| {
        dialog_clone.close();
    });

    box_container.append(&title_label);
    box_container.append(&scroll);
    box_container.append(&close_btn);

    dialog.set_child(Some(&box_container));
    dialog.present();
}
