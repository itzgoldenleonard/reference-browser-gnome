use crate::athn_document;
use crate::athn_document::{Document, ParserState};
use crate::window::input::Input;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::Leaflet;
use core::fmt::Debug;
use glib::subclass::InitializingObject;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, Label, ListBox, SearchEntry, Stack, TextTagTable, ScrolledWindow, TextBuffer};
use std::cell::RefCell;
use std::fs;
use url::Url;

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
    pub toaster: TemplateChild<adw::ToastOverlay>,
    #[template_child]
    pub canvas: TemplateChild<ListBox>,
    #[template_child]
    pub text_block_tag_table: TemplateChild<TextTagTable>,
    #[template_child]
    pub server_error_window: TemplateChild<ScrolledWindow>,
    #[template_child]
    pub server_error_buffer: TemplateChild<TextBuffer>,
    #[property(get, set = Self::go_to_url)]
    pub uri: RefCell<String>,
    pub form_data: RefCell<Vec<Vec<Input>>>,
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

fn validate_url(url: &str) -> Result<Url, url::ParseError> {
    let has_supported_protocol = url.starts_with("https://") || url.starts_with("file://");
    if has_supported_protocol {
        Url::parse(url)
    } else {
        let url = format!("https://{}", url);
        Url::parse(&url)
    }
}

fn get_document(url: &Url) -> Result<String, String> {
    match url.scheme() {
        "https" => get_document_by_https(url).map_err(|e| e.to_string()),
        "file" => get_document_by_file(url).map_err(|e| e.to_string()),
        _ => Err("Unsupported protocol".to_string()),
    }
}

fn get_document_by_file(url: &Url) -> Result<String, std::io::Error> {
    // TODO: use url.to_file_path()
    fs::read_to_string(url.path())
}

fn get_document_by_https(url: &Url) -> reqwest::Result<String> {
    let https_client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = https_client.get(url.clone()).send()?;

    response.text()
}

#[gtk::template_callbacks]
impl Window {
    fn go_to_url(&self, input: String) {
        self.stack.set_visible_child_name("canvas");
        let start_time = std::time::Instant::now();

        self.search_entry.set_text(&input);

        let url = validate_url(&input);
        let url = match url {
            Err(e) => return self.set_request_error(&e.to_string()),
            Ok(val) => val,
        };

        // Sets the actual value in the window object, syntax referenced from https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/derive.Properties.html#example
        *self.uri.borrow_mut() = url.to_string();
        *self.form_data.borrow_mut() = vec![];

        let response = get_document(&url);
        let response = match response {
            Err(e) => return self.set_request_error(&e),
            Ok(val) => val,
        };

        let request_time = start_time.elapsed();

        self.render_document(&response, &url);

        // Timing stuff, dont mind me
        let total_time = start_time.elapsed();
        println!(
            "
        ╭─────────────────┬─────────
        │ Request timing breakdown:
        ├─────────────────┼─────────
        │ Network fetch:  │ {:?}
        │ Rendering:      │ {:?}
        ├─────────────────┼─────────
        │ \x1b[1mTotal\x1b[0m           │ \x1b[1m{:?}\x1b[0m
        ╰─────────────────┴─────────
        ",
            request_time,
            total_time - request_time,
            total_time
        );
    }

    pub fn render_document(&self, document_string: &str, base_url: &Url) {
        let document = athn_document::parse(
            document_string.lines(),
            Document::builder(),
            ParserState::default(),
        );
        let document = match document {
            Err(e) => {
                eprintln!("{e}");
                return self.stack.set_visible_child_name("parse-error");
            }
            Ok(val) => val.build(),
        };

        self.server_error_window.set_visible(false);
        self.obj().render(document, base_url);
    }

    fn set_request_error(&self, err_message: &str) {
        self.stack.set_visible_child_name("request-error");
        self.request_error.set_label(err_message);
    }

    pub fn is_form_valid(&self, form_idx: usize) -> bool {
        let data = self.form_data.borrow();
        data[form_idx].iter().find(|e| e.valid == false).is_none()
    }

    #[template_callback]
    fn on_search_entry_activate(&self, search_entry: &gtk::SearchEntry) {
        self.obj().set_uri(search_entry.text());
    }

    #[template_callback]
    fn on_parse_error_button_clicked(&self, _button: &gtk::Button) {
        let uri = self.obj().uri();
        let launcher = gtk::UriLauncher::new(&uri);
        launcher.launch(None::<&gtk::Window>, None::<&gtk::gio::Cancellable>, |_| ());
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
    fn on_show_settings_pressed(&self, _button: &gtk::Button) {
        self.stack.set_visible_child_name("settings");
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
        self.obj().set_uri(entry_url);
    }

    #[template_callback]
    fn on_language_preference_update(&self, entry: &gtk::Entry) {
        println!("{}", entry.text());
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
