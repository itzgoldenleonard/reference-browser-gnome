mod imp;

use crate::athn_document::AthnDocument;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{gio, glib, Label};
use adw::Application;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn render(&self, document: AthnDocument) {
        let title = Label::new(Some(document.metadata.title.as_str()));
        self.imp().canvas.append(&title);
    }
}
