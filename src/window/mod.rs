mod imp;

use crate::athn_document::Document;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::Application;
use glib::Object;
use gtk::{gio, glib, Label, Separator};

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

    pub fn render(&self, document: Document) {
        // Show title metadata attribute
        let title = Label::builder()
            .label(document.metadata.title.as_str())
            .halign(gtk::Align::Start)
            .build();
        title.add_css_class("large-title");
        self.imp().canvas.append(&title);

        // Show author and license metadata attribute
        let author_formatter = |acc: String, val: &String| {
            if acc.is_empty() {
                val.to_string()
            } else {
                format!("{acc}, {val}")
            }
        };
        let license_formatter = |acc: String, val: &String| {
            if acc.is_empty() {
                val.to_string()
            } else {
                format!("{acc}, {val}")
            }
        };
        match (document.metadata.author, document.metadata.license) {
            (Some(author), Some(license)) => {
                let metaline = Label::builder()
                    .label(format!(
                        "By: {}. License: {}",
                        author.iter().fold(String::new(), author_formatter),
                        license.iter().fold(String::new(), license_formatter)
                    ))
                    .halign(gtk::Align::Start)
                    .build();
                self.imp().canvas.append(&metaline);
            }
            (Some(author), None) => {
                let metaline = Label::builder()
                    .label(format!(
                        "By: {}",
                        author.iter().fold(String::new(), author_formatter)
                    ))
                    .halign(gtk::Align::Start)
                    .build();
                self.imp().canvas.append(&metaline);
            }
            (None, Some(license)) => {
                let metaline = Label::builder()
                    .label(format!(
                        "License: {}",
                        license.iter().fold(String::new(), license_formatter)
                    ))
                    .halign(gtk::Align::Start)
                    .build();
                self.imp().canvas.append(&metaline);
            }
            (None, None) => (),
        }

        // Show subtitle if there is one
        if document.metadata.subtitle.is_some() {
            let subtitle = Label::builder()
                .label(document.metadata.subtitle.unwrap_or("Default subtitle".to_string()))
                .halign(gtk::Align::Start)
                .build();
            self.imp().canvas.append(&subtitle);
        }

        // Horizontal seperator between metadata and main sections
        let metadata_separator = Separator::builder().build();
        self.imp().canvas.append(&metadata_separator);

        // Render main section
        for line in document.main_lines {
            use crate::athn_document::MainLine::*;
            match line {
                TextLine(content) => {
                    let text_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .build();

                    self.imp().canvas.append(&text_obj);
                },
                HeadingLine(level, content) => {
                    let heading_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .build();
                    use crate::athn_document::HeadingLevel::*;
                    let heading_class = match level {
                        One => "title-1",
                        Two => "title-2",
                        Three => "title-3",
                        Four => "title-4",
                        Five => "heading",
                        Six => "caption-heading",
                    };
                    heading_obj.add_css_class(heading_class);
                    self.imp().canvas.append(&heading_obj);
                }
            }
        };
    }
}
