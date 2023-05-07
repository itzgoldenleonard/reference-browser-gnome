mod window;

use adw::prelude::*;
use adw::Application;
use gtk::gio;
use window::Window;

const APP_ID: &str = "org.athn.browser.gnome";

fn main() {
    // Register and include ui
    gio::resources_register_include!("browser.gresource").expect("Failed to register resources.");

    let application = Application::builder().application_id(APP_ID).build();

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
