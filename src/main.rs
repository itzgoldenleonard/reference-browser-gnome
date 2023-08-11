mod athn_document;
mod submit;
mod window;
//mod integer;
mod date;
mod email;
mod file;

use adw::prelude::*;
use adw::Application;
use gtk::gio;
use window::Window;

const APP_ID: &str = "org.athn.browser.gnome";

fn main() {
    // Register and include ui
    gio::resources_register_include!("browser.gresource").expect("Failed to register resources.");

    let application = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    application.connect_activate(build_ui);

    application.connect_open(open_file);

    application.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}

fn open_file(app: &Application, files: &[gio::File], _hint: &str) {
    let uri = files[0].uri();

    let window = Window::new(app);
    window.present();
    window.set_uri(uri);
}

/* Useful documentation
 * gtk4_rs book: https://gtk-rs.org/gtk4-rs/git/book
 * gtk4_rs documentation: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/index.html
 * libadwaita (rust) documentation: https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/0.4/docs/libadwaita/index.html
 * gtk4 documentation: https://docs.gtk.org/gtk4/index.html
 * libadwaita documentation: https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.3/index.html
 * Gnome HIG: https://developer.gnome.org/hig/index.html
 */
