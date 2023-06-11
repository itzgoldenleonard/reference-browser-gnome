use crate::athn_document::{parse, Document, ParserState};
use url::Url;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, ListBox};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/athn/browser/gnome/window.ui")]
pub struct Window {
    #[template_child]
    pub canvas: TemplateChild<ListBox>,
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
    fn on_search_entry_activate(&self, search_entry: &gtk::SearchEntry) {
        let start_time = std::time::Instant::now();
        // Extract the query from the search entry and parse it into a URL
        let url = Url::parse(&search_entry.text().to_string()).unwrap();

        // Make a reqwest client that doesnt validate certificates
        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build a client");

        // Make the actual request
        let response = client
            .get(url.clone())
            .send()
            .expect("Failed to make a request to the URL");
        let request_time = start_time.elapsed();

        // Extract and parse the athn document data from the Response and pass it to the render function
        let document = parse(response.text().unwrap().as_str().lines(), Document::builder(), ParserState::default()).unwrap().build();
        let parse_time = start_time.elapsed();
        self.obj().render(document, url);

        let total_time = start_time.elapsed();
        println!("
        ╭─────────────────┬─────────
        │ Request timing breakdown:
        ├─────────────────┼─────────
        │ Network fetch:  │ {:?}
        │ Document parse: │ {:?}
        │ Rendering:      │ {:?}
        ├─────────────────┼─────────
        │ \x1b[1mTotal\x1b[0m           │ \x1b[1m{:?}\x1b[0m
        ╰─────────────────┴─────────
        ",
        request_time,
        parse_time - request_time,
        total_time - parse_time,
        total_time
        );
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
