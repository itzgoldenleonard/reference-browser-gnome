mod imp;

use crate::athn_document::Document;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{ActionRow, Application, ExpanderRow};
use glib::Object;
use gtk::{gio, glib, Label, ListBox, Separator, TextBuffer, TextView};
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
                        "By: {}. License{}: {}",
                        author.iter().fold(String::new(), author_formatter),
                        if license.len() > 1 { "s" } else { "" },
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
                        "License{}: {}",
                        if license.len() > 1 { "s" } else { "" },
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

        let level_number = |l: &crate::athn_document::line_types::Level| {
            use crate::athn_document::line_types::Level::*;
            match l {
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
            }
        };

        // Render main section
        for line in document.main {
            use crate::athn_document::line_types::MainLine::*;
            match line {
                TextLine(content) => {
                    // Change ATHN formatting into pango markup
                    let content: String = content
                        .as_str()
                        .split("\\r")
                        .map(|s| {
                            // The whole thing with the vector with sorting and filtering and stuff
                            // is needed because pango needs the end tags to be in the reverse
                            // order of the start tags
                            let mut states = vec![
                                ("</b>", s.find("\\b")),
                                ("</i>", s.find("\\i")),
                                ("</tt>", s.find("\\p")),
                            ]
                            .iter()
                            .filter_map(|state| match state.1 {
                                None => None,
                                Some(n) => Some((state.0, n)),
                            })
                            .collect::<Vec<(&str, usize)>>();

                            // This is probably not the most efficient way to do this
                            let s = s.replacen("\\b", "<b>", 1);
                            let s = s.replacen("\\i", "<i>", 1);
                            let s = s.replacen("\\p", "<tt>", 1);
                            let s = s.replace("\\b", "");
                            let s = s.replace("\\i", "");
                            let mut s = s.replace("\\p", "");

                            states.sort_unstable_by_key(|k| k.1);
                            states.reverse();
                            s.push_str(
                                states
                                    .iter()
                                    .map(|state| state.0.to_owned())
                                    .collect::<String>()
                                    .as_str(),
                            );

                            s
                        })
                        .collect();

                    let text_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .use_markup(true)
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
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
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
                        .build();

                    self.imp().canvas.append(&link_obj);
                }

                PreformattedLine(_, content) => {
                    // I'm using textviews to hopefully one day join adjacent lines into 1 code block
                    let text_obj = TextView::builder()
                        .editable(false)
                        .monospace(true)
                        .cursor_visible(false)
                        .build();

                    let buffer = TextBuffer::builder().text(content).build();

                    text_obj.set_buffer(Some(&buffer));

                    text_obj.add_css_class("monospace");
                    self.imp().canvas.append(&text_obj);
                }

                SeparatorLine => {
                    self.imp().canvas.append(&Separator::builder().build());
                }

                UListLine(level, content) => {
                    let list_point_obj = Label::builder()
                        .label(format!("• {}", content))
                        .halign(gtk::Align::Start)
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
                        .margin_start(level_number(&level) * 12) // It's probably not a good idea to use a fixed number like this for indentation
                        .build();

                    self.imp().canvas.append(&list_point_obj);
                }

                OListLine(level, bullet, content) => {
                    let bullet_width: i32 = 5 * bullet.len() as i32; // Crude approximation, could definitely be better

                    let list_point_obj = Label::builder()
                        .label(format!("{} {}", bullet, content))
                        .halign(gtk::Align::Start)
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
                        .margin_start(std::cmp::max(level_number(&level) * 12 - bullet_width, 0))
                        .build();

                    self.imp().canvas.append(&list_point_obj);
                }

                DropdownLine(label, content) => {
                    let list_box = ListBox::builder()
                        .selection_mode(gtk::SelectionMode::None)
                        .build();
                    list_box.add_css_class("boxed-list");

                    let content_row = ActionRow::builder().title_lines(0).title(content).build();

                    let expander = ExpanderRow::builder().title(label).build();

                    expander.add_row(&content_row);
                    list_box.append(&expander);

                    self.imp().canvas.append(&list_box);
                }

                AdmonitionLine(admonition_type, content) => {
                    use crate::athn_document::line_types::AdmonitionType::*;

                    let admonition_label = Label::builder()
                        .label(format!(
                            "{} {}",
                            match admonition_type {
                                // It would be better to use a proper icon, but then again it might
                                // also be better to use something that's not a button
                                Note => "ℹ️",
                                Warning => "⚠️",
                                Danger => "⛔",
                            },
                            content
                        ))
                        .halign(gtk::Align::Start)
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
                        .build();

                    let admonition_obj = gtk::Button::builder()
                        .child(&admonition_label)
                        .can_focus(false)
                        .can_target(false)
                        .focus_on_click(false)
                        .focusable(false)
                        .build();

                    match admonition_type {
                        Warning => admonition_obj.add_css_class("warning"),
                        Danger => admonition_obj.add_css_class("error"),
                        _ => (),
                    }

                    self.imp().canvas.append(&admonition_obj);
                }

                HeadingLine(level, content) => {
                    let heading_obj = Label::builder()
                        .label(content)
                        .halign(gtk::Align::Start)
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
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

                QuoteLine(content) => {
                    let text_obj = Label::builder()
                        .label(format!("“<i>{}</i>”", content))
                        .halign(gtk::Align::Start)
                        .wrap(true)
                        .wrap_mode(gtk::pango::WrapMode::WordChar)
                        .use_markup(true)
                        .build();

                    self.imp().canvas.append(&text_obj);
                }
                _ => (),
            }
        }
    }
}
