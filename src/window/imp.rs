use crate::athn_document::{parse, Document, ParserState};
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::Leaflet;
use glib::subclass::InitializingObject;
use glib::{Properties, Value, ParamSpec};
use gtk::{glib, CompositeTemplate, Label, ListBox, SearchEntry, Stack};
use url::Url;
use std::cell::RefCell;

#[derive(Properties, CompositeTemplate, Default)]
#[template(resource = "/org/athn/browser/gnome/window.ui")]
#[properties(wrapper_type = super::Window)]
pub struct Window {
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
    #[template_child]
    pub header: TemplateChild<ListBox>,
    #[template_child]
    pub search_entry: TemplateChild<SearchEntry>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub request_error: TemplateChild<Label>,
    #[template_child]
    pub canvas: TemplateChild<ListBox>,
    //TODO: Make this custom property's setter do all of the request and parsing stuff
    #[property(get, set)]
    pub base_url: RefCell<String>,
}

// Boilerplate
#[glib::object_subclass]
impl ObjectSubclass for Window {
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

fn validate_url(url: String) -> Result<Url, url::ParseError> {
    let has_supported_protocol = url.starts_with("https://");
    if has_supported_protocol {
        Url::parse(&url)
    } else {
        let url = format!("https://{}", url);
        Url::parse(&url)
    }
}

#[gtk::template_callbacks]
impl Window {
    fn set_request_error(&self, err_message: &str) {
        self.stack.set_visible_child_name("request-error");
        self.request_error.set_label(err_message);
    }

    fn show_parse_error(&self) {
        self.stack.set_visible_child_name("parse-error");
    }

    #[template_callback]
    fn on_search_entry_activate(&self, search_entry: &gtk::SearchEntry) {
        self.stack.set_visible_child_name("canvas");
        let start_time = std::time::Instant::now();

        let url = validate_url(search_entry.text().into());
        let url = match url {
            Err(e) => return self.set_request_error(&e.to_string()),
            Ok(val) => val,
        };
        self.obj().set_base_url(Into::<String>::into(url.clone()));

        let https_client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build();
        let https_client = match https_client {
            Err(_) => return self.set_request_error("TLS backend cannot be initialized, or the resolver cannot load the system configuration"),
            Ok(val) => val,
        };

        let response = https_client.get(url.clone()).send();
        let response = match response {
            Ok(val) => val,
            Err(e) => return self.set_request_error(&e.to_string()),
        };
        let response = match response.text() {
            Ok(val) => val,
            Err(e) => return self.set_request_error(&e.to_string()),
        };

        let request_time = start_time.elapsed();

        // Extract and parse the athn document data from the Response and pass it to the render function
        let document = parse(
            response.as_str().lines(),
            Document::builder(),
            ParserState::default(),
        );
        let document = match document {
            Err(e) => {
                eprintln!("{e}");
                return self.show_parse_error();
            }
            Ok(val) => val.build(),
        };

        let parse_time = start_time.elapsed();

        self.obj().render(document, url);

        let total_time = start_time.elapsed();
        println!(
            "
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

    #[template_callback]
    fn on_parse_error_button_clicked(&self, _button: &gtk::Button) {
        let uri = self.obj().base_url();
        let launcher = gtk::UriLauncher::new(uri.as_str());
        launcher.launch(None::<&gtk::Window>, None::<&gio::Cancellable>, |_| ());
    }

    #[template_callback]
    fn on_hide_header_button_clicked(&self, _button: &gtk::Button) {
        self.leaflet.navigate(adw::NavigationDirection::Forward);
    }

    #[template_callback]
    fn on_show_header_button_clicked(&self, _button: &gtk::Button) {
        self.leaflet.navigate(adw::NavigationDirection::Back);
    }

    #[template_callback]
    fn on_header_entry_activated(&self, row: &gtk::ListBoxRow) {
        let row_label = match row.child().and_downcast::<Label>() {
            Some(row_label) => row_label,
            None => return eprintln!("A ListBoxRow without a Label in the 'header' ListBox was clicked. This is a bug, please report it to: https://github.com/itzgoldenleonard/reference-browser-gnome/issues"),
        };
        let entry_url = match row_label.tooltip_text() {
            Some(entry_url) => entry_url,
            None => return eprintln!("A header entry without a url in its tooltip was clicked. This is a bug, please report it to: https://github.com/itzgoldenleonard/reference-browser-gnome/issues"),
        };
        self.go_to_url(&entry_url);
    }

    fn go_to_url(&self, url: &str) {
        // This function doesnt check the url's validity, it would be a good idea to do that before
        // calling this
        self.search_entry.delete_text(0, i32::MAX);
        self.search_entry.insert_text(url, &mut 0);
        self.search_entry.emit_activate();
    }
}

// More boilerplate
impl ObjectImpl for Window {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
