use crate::athn_document::AthnDocument;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
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
    fn on_search_entry_activate(search_entry: &gtk::SearchEntry) {
        println!("Searched: {}", search_entry.text());
        let url = search_entry.text().to_string();

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build a client");

        let response = client
            .get(url)
            .send()
            .expect("Failed to make a request to the URL");

        let document = AthnDocument::from_str(response.text().unwrap().as_str()).unwrap();
        // Placeholder just to see that it works, it will be replaced with a call to the rendering
        // function
        println!("Title: {}", document.metadata.title);
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

// Trait shared by all adw application windows
impl AdwApplicationWindowImpl for Window {}


/*
let label = Label::new(document.metadata.title.to_string());
list_box.append(&label);
*/
