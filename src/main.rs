mod app;
mod components;
mod utils;

use gtk::gdk;
use relm4::RelmApp;
use app::model::AppModel;

fn main() {
    let app = RelmApp::new("com.herramientas.visor");

    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        headerbar {
            border-bottom: none;
            box-shadow: none;
        }
        .clean-viewport {
            margin: 0px;
            padding: 0px;
            border: none;
            background-color: #000000;
        }
        .osd.circular {
            border-radius: 9999px;
            padding: 8px;
            background-color: rgba(0, 0, 0, 0.6);
            color: #ffffff;
        }
        .osd.circular:hover {
            background-color: rgba(0, 0, 0, 0.85);
        }
        ",
    );

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    app.run::<AppModel>(());
}
