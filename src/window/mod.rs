mod imp;

use crate::athn_document::Document;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::Application;
use glib::Object;
use gtk::{gio, glib, Label, Separator};
use url::Url;

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

    pub fn render(&self, document: Document, base_url: Url) {
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
                .label(
                    document
                        .metadata
                        .subtitle
                        .unwrap_or("Default subtitle".to_string()),
                )
                .halign(gtk::Align::Start)
                .build();
            self.imp().canvas.append(&subtitle);
        }

        // Horizontal seperator between metadata and main sections
        self.imp().canvas.append(&Separator::builder().build());

        // Render main section
        for line in document.main {
            use crate::athn_document::line_types::MainLine::*;
            match line {
                TextLine(content) => {
                    let text_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .build();

                    self.imp().canvas.append(&text_obj);
                }


                LinkLine(crate::athn_document::line_types::Link { url, label }) => {
                    let url_parsed = match Url::parse(&url) {
                        Ok(u) => Some(u),
                        Err(url::ParseError::RelativeUrlWithoutBase) => {
                            Some(base_url.join(&url).unwrap())
                        }
                        Err(_) => None,
                    };

                    let true_label = if label.is_none() {
                        url
                    } else {
                        label.unwrap_or_default()
                    };

                    let link_obj = Label::builder()
                        .label(match url_parsed {
                            Some(url_parsed) => {
                                format!("<a href=\"{}\">{}</a>", url_parsed, true_label,)
                            }
                            None => format!("<a href=\"\">{}</a> <i>(Invalid URL)</i>", true_label),
                        })
                        .use_markup(true)
                        .halign(gtk::Align::Start)
                        .build();

                    self.imp().canvas.append(&link_obj);
                }


                PreformattedLine(_, content) => {
                    let text_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .build();

                    text_obj.add_css_class("monospace");
                    self.imp().canvas.append(&text_obj);
                }


                SeparatorLine => {
                    self.imp().canvas.append(&Separator::builder().build());
                }


                HeadingLine(level, content) => {
                    let heading_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .build();
                    use crate::athn_document::line_types::Level::*;
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
                _ => (),
            }
        }
    }
}
