use glib::subclass::InitializingObject;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/athn/browser/gnome/window.ui")]
pub struct Window {
    //#[template_child]
    //pub button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "AthnBrowserAppWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_helloworldrow_activated(row: &adw::ActionRow) {
        println!("Hello world!");
        row.set_subtitle("Hello world!");
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window { }

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

// Trait shared by all adw application windows
impl AdwApplicationWindowImpl for Window {}
