mod imp;
mod input;

use crate::athn_document::form;
use crate::athn_document::{line_types, line_types::MainLine, Document, Metadata};
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{ActionRow, Application, ButtonContent, ExpanderRow};
use glib::{closure_local, GString, Object};
use gtk::{
    gio, glib, Label, ListBox, ListBoxRow, Orientation::Horizontal, Separator, SpinButton,
    TextBuffer, TextView,
};
use input::*;
use url::Url;
// Custom widgets
use crate::submit::SubmitFormField;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn render(&self, document: Document, base_url: &Url) {
        clear_list_box(&self.imp().canvas);
        clear_list_box(&self.imp().header);

        self.render_metadata(document.metadata);

        for line in document.main {
            self.render_main_line(line, base_url);
        }

        if let Some(header) = document.header {
            for line in header {
                self.imp()
                    .header
                    .append(&create_header_entry(line, base_url));
            }
        }

        if let Some(footer) = document.footer {
            self.render_footer_section(footer, base_url);
        }

        list_box_map(&self.imp().canvas, |row, _| row.set_activatable(false));
    }

    fn render_footer_section(&self, footer: Vec<line_types::FooterLine>, base_url: &Url) {
        use crate::athn_document::line_types::FooterLine::*;

        let footer_separator = Separator::builder().margin_top(26).build();
        self.imp().canvas.append(&footer_separator);

        for line in footer {
            match line {
                TextLine(content) => self.imp().canvas.append(&create_text_line(content)),
                LinkLine(link) => self.imp().canvas.append(&create_link_line(link, base_url)),
            }
        }
    }

    fn current_code_block(&self) -> Option<TextView> {
        // Returns the code block at the end of the canvas, returns 'None' if the last child of the canvas is not a code block
        self.imp()
            .canvas
            .last_child()?
            .last_child()
            .and_downcast::<TextView>()
    }

    fn render_main_line(&self, line: MainLine, base_url: &Url) {
        use MainLine::*;
        macro_rules! append {
            ($x:expr) => {
                self.imp().canvas.append(&$x)
            };
        }
        match line {
            TextLine(content) => append!(create_text_line(content)),
            LinkLine(link) => append!(create_link_line(link, base_url)),
            PreformattedLine(_, content) => match self.current_code_block() {
                None => append!(create_code_block(content)),
                Some(code_block) => {
                    code_block
                        .buffer()
                        .insert_at_cursor(format!("\n{}", content).as_str());
                }
            },
            SeparatorLine => append!(Separator::new(Horizontal)),
            UListLine(level, content) => append!(create_ulist_line(level, content)),
            OListLine(level, bullet, content) => append!(create_olist_line(level, bullet, content)),
            DropdownLine(label, content) => append!(create_dropdown_line(label, content)),
            AdmonitionLine(type_, content) => append!(create_admonition_line(type_, content)),
            HeadingLine(level, content) => append!(create_heading_line(level, content)),
            QuoteLine(content) => append!(create_quote_line(content)),
            FormFieldLine(_form_id, line) => {
                self.render_form_field(line, base_url);
            }
        }
    }

    fn render_form_field(&self, field: form::FormField, base_url: &Url) {
        use form::FormField::*;
        macro_rules! append {
            ($x:expr) => {
                self.imp().canvas.append(&$x)
            };
        }
        match field {
            Submit(id, field) => append!(create_submit_form_field(self, id, field, base_url)),
            Integer(id, field) => append!(create_submit_int_field(self, id, field)),
            Float(id, field) => append!(create_submit_float_field(self, id, field)),
            //String(id, field) => append!(create_submit_string_field(self, id, field)),
            _ => (),
        }
    }

    fn render_metadata(&self, metadata: Metadata) {
        self.imp()
            .canvas
            .append(&create_document_title(metadata.title));

        if let Some(metaline) = create_metaline(metadata.author, metadata.license) {
            self.imp().canvas.append(&metaline);
        }

        if let Some(subtitle) = create_subtitle(metadata.subtitle) {
            self.imp().canvas.append(&subtitle);
        }

        self.imp().canvas.append(&Separator::new(Horizontal));
    }
}

fn create_submit_int_field(window: &Window, id: form::ID, field: form::IntField) -> SpinButton {
    let min = field.min.unwrap_or(i64::MIN);
    let max = field.max.unwrap_or(i64::MAX);
    let step = field.step.unwrap_or(1);

    let widget = SpinButton::with_range(min as f64, max as f64, step as f64);
    widget.set_tooltip_text(Some(&id.id_cloned()));
    widget.set_has_tooltip(false);
    let default = field.global.default.unwrap_or(0);

    let new_input_data = Input {
        id,
        value: InputTypes::Int(Some(default)),
    };
    window.imp().form_data.borrow_mut().push(new_input_data);

    widget.set_value(default as f64);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "value-changed",
        false,
        closure_local!(@watch window => move |entry: &SpinButton| {
            let id = form::ID::new(entry.tooltip_text().unwrap().as_str()).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            override_element_by_id(&mut all_data, id, InputTypes::Int(Some(entry.value_as_int().into())));
        }),
    );

    widget
}

fn create_submit_float_field(window: &Window, id: form::ID, field: form::FloatField) -> SpinButton {
    let min = field.min.unwrap_or(f64::MIN);
    let max = field.max.unwrap_or(f64::MAX);
    let step = field.step.unwrap_or(0.001);

    let widget = SpinButton::with_range(min, max, step);
    widget.set_tooltip_text(Some(&id.id_cloned()));
    widget.set_has_tooltip(false);
    let default = field.global.default.unwrap_or(0.);

    let new_input_data = Input {
        id,
        value: InputTypes::Float(Some(default)),
    };
    window.imp().form_data.borrow_mut().push(new_input_data);

    widget.set_value(default as f64);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "value-changed",
        false,
        closure_local!(@watch window => move |entry: &SpinButton| {
            let id = form::ID::new(entry.tooltip_text().unwrap().as_str()).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            override_element_by_id(&mut all_data, id, InputTypes::Float(Some(entry.value())));
        }),
    );

    widget
}

/// Returns an error if the id doesnt exist
fn override_element_by_id(
    vector: &mut Vec<Input>,
    id: form::ID,
    new_value: InputTypes,
) -> Result<(), ()> {
    let idx = vector.iter().enumerate().find(|&x| *x.1 == id).ok_or(())?.0;
    vector[idx].value = new_value;
    Ok(())
}

fn create_submit_form_field(
    window: &Window,
    id: form::ID,
    field: form::SubmitField,
    base_url: &Url,
) -> SubmitFormField {
    let url = validate_url(&field.destination, base_url).ok();

    let widget = SubmitFormField::new(id, field.label, url, field.redirect);

    widget.connect_closure(
        "data-request",
        false,
        closure_local!(@watch window => move |button: SubmitFormField| {
            //button.set_serialized_data(serialize_form_data(&window.imp().form_data.borrow()));
            button.set_serialized_data(serde_json::to_string(&window.imp().form_data).unwrap());
        }),
    );

    widget.connect_closure(
        "submit-success",
        false,
        closure_local!(@watch window, @strong base_url => move |button: SubmitFormField, body: std::string::String| {
            if button.redirect() {
                window.imp().render_document(&body, &base_url);
            };
            let toast = adw::Toast::new("Successfully submitted the form");
            window.imp().toaster.add_toast(toast);
        }),
    );

    widget.connect_closure(
        "submit-error",
        false,
        closure_local!(@watch window => move |_button: SubmitFormField, message: std::string::String| {
            eprintln!("Failed to submit with error: {message}");

            let toast = adw::Toast::new(format!("Failed to submit with error: {message}").as_str());
            toast.set_timeout(0);
            window.imp().toaster.add_toast(toast);
            if let Some(toast_widget) = window.imp().toaster.last_child() {
                toast_widget.add_css_class("error");
            }
        }),
    );

    widget
}

fn list_box_map(list_box: &ListBox, map: fn(widget: &ListBoxRow, parent: &ListBox)) {
    let original_selection_mode = list_box.selection_mode();

    list_box.set_selection_mode(gtk::SelectionMode::Multiple);
    list_box.select_all();
    for widget in list_box.selected_rows() {
        map(&widget, list_box);
    }
    list_box.set_selection_mode(original_selection_mode);
}

fn clear_list_box(list_box: &ListBox) {
    // When I gtk4.12 I can use this https://docs.gtk.org/gtk4/method.ListBox.remove_all.html
    list_box_map(list_box, |widget, list_box| list_box.remove(widget))
}

fn create_document_title(title: impl Into<GString>) -> Label {
    let widget = Label::builder()
        .label(title)
        .halign(gtk::Align::Start)
        .build();
    widget.add_css_class("large-title");
    widget
}

fn format_author_string(author_list: Option<Vec<String>>) -> Option<String> {
    let author_list = author_list?;
    Some(format!(
        "By: {}",
        author_list.iter().fold(String::new(), |acc, val| {
            if acc.is_empty() {
                val.to_owned()
            } else {
                format!("{acc}, {val}")
            }
        })
    ))
}

fn format_license_string(license_list: Option<Vec<String>>) -> Option<String> {
    let license_list = license_list?;
    Some(format!(
        "License{}: {}",
        if license_list.len() > 1 { "s" } else { "" },
        license_list.iter().fold(String::new(), |acc, val| {
            if acc.is_empty() {
                val.to_owned()
            } else {
                format!("{acc}, {val}")
            }
        })
    ))
}

fn create_metaline(author: Option<Vec<String>>, license: Option<Vec<String>>) -> Option<Label> {
    if author.is_none() && license.is_none() {
        return None;
    };

    let author_string = format_author_string(author);
    let license_string = format_license_string(license);
    let both_are_some = license_string.is_some() && author_string.is_some();

    let label = format!(
        "{}{}{}",
        author_string.unwrap_or_default(),
        if both_are_some { ". " } else { "" },
        license_string.unwrap_or_default()
    );

    Some(
        Label::builder()
            .label(label)
            .halign(gtk::Align::Start)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .build(),
    )
}

fn create_subtitle(subtitle: Option<String>) -> Option<Label> {
    Some(
        Label::builder()
            .label(subtitle?)
            .halign(gtk::Align::Start)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .build(),
    )
}

fn escape_pango_markup(s: String) -> String {
    s.replace("<", "&lt;")
}

fn convert_athn_formatting_to_pango(content: String) -> String {
    //TODO: Perhaps make this a little less monolithic, that escaping pango markup could be a good
    //function to have
    content
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
            let s = escape_pango_markup(s.into());
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
        .collect()
}

fn create_text_line(content: String) -> Label {
    let content = convert_athn_formatting_to_pango(content);
    Label::builder()
        .label(content)
        .halign(gtk::Align::Start)
        .use_markup(true)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .build()
}

fn validate_url(url: &String, base_url: &Url) -> Result<String, url::ParseError> {
    let url_may_be_relative = Url::parse(url);
    if url_may_be_relative == Err(url::ParseError::RelativeUrlWithoutBase) {
        base_url.join(url).map(|u| u.into())
    } else {
        url_may_be_relative.map(|u| u.into())
    }
}

fn create_link_line(link: line_types::Link, base_url: &Url) -> Label {
    let label = link.label.unwrap_or(link.url.clone());
    let label = escape_pango_markup(label);

    let url = validate_url(&link.url, base_url);
    let url_is_invalid = url.is_err();

    let label_markup = format!(
        "<a href=\"{}\">{}</a>",
        url.clone().unwrap_or_default(),
        label
    );
    let tooltip: String = url.unwrap_or("Broken url".into());

    let widget = Label::builder()
        .label(label_markup)
        .use_markup(true)
        .tooltip_text(tooltip)
        .halign(gtk::Align::Start)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .build();
    if url_is_invalid {
        widget.add_css_class("dim-label");
    }
    widget
}

fn create_code_block(content: String) -> TextView {
    let buffer = TextBuffer::builder().text(content).build();

    let widget = TextView::builder()
        .editable(false)
        .monospace(true)
        .cursor_visible(false)
        .build();
    widget.set_buffer(Some(&buffer));
    widget.add_css_class("monospace");
    // This is my hacky solution to the problem of single preformatted
    // lines (if there arent any multi line code blocks in the rest of the
    // document) not rendering properly until the window is resized
    widget.set_height_request(20);
    widget
}

fn calculate_indentation_value(level: line_types::Level) -> i32 {
    // TODO: It's probably not a good idea to use a fixed number like this for indentation
    use line_types::Level::*;
    let level = match level {
        One => 1,
        Two => 2,
        Three => 3,
        Four => 4,
        Five => 5,
        Six => 6,
    };
    level * 12
}

fn create_ulist_line(level: line_types::Level, content: String) -> Label {
    Label::builder()
        .label(format!("• {}", content))
        .halign(gtk::Align::Start)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .margin_start(calculate_indentation_value(level))
        .build()
}

fn calculate_indentation_value_with_bullet(level: line_types::Level, bullet: String) -> i32 {
    let bullet_width = 5 * bullet.len() as i32; // TODO: Crude approximation, could definitely be better
    std::cmp::max(calculate_indentation_value(level) - bullet_width, 0)
}

fn create_olist_line(level: line_types::Level, bullet: String, content: String) -> Label {
    Label::builder()
        .label(format!("{} {}", bullet, content))
        .halign(gtk::Align::Start)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .margin_start(calculate_indentation_value_with_bullet(level, bullet))
        .build()
}

fn create_dropdown_line(label: String, content: String) -> ListBox {
    let widget = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    widget.add_css_class("boxed-list");

    let expander = ExpanderRow::builder().title(label).build();
    let content_row = ActionRow::builder().title_lines(0).title(content).build();
    expander.add_row(&content_row);
    widget.append(&expander);
    widget
}

fn create_admonition_line(type_: line_types::AdmonitionType, content: String) -> gtk::Button {
    use line_types::AdmonitionType::*;

    let label = ButtonContent::builder()
        .label(content)
        .icon_name(match type_ {
            //TODO: Add proper icons
            Note => "question-symbolic",
            Warning => "warning-symbolic",
            Danger => "error-symbolic",
        })
        .build();

    if let Some(l) = label.last_child().and_downcast::<Label>() {
        l.set_wrap(true);
    }

    let widget = gtk::Button::builder()
        .child(&label)
        .can_focus(false)
        .can_target(false)
        .focus_on_click(false)
        .focusable(false)
        .build();

    match type_ {
        Warning => widget.add_css_class("warning"),
        Danger => widget.add_css_class("error"),
        _ => (),
    }
    widget
}

fn create_heading_line(level: line_types::Level, content: String) -> Label {
    let widget = Label::builder()
        .label(content)
        .halign(gtk::Align::Start)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .build();
    use line_types::Level::*;
    let heading_class = match level {
        One => "title-1",
        Two => "title-2",
        Three => "title-3",
        Four => "title-4",
        Five => "heading",
        Six => "caption-heading",
    };
    widget.add_css_class(heading_class);
    widget
}

fn create_quote_line(content: String) -> Label {
    Label::builder()
        .label(format!("“<i>{}</i>”", escape_pango_markup(content)))
        .halign(gtk::Align::Start)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .use_markup(true)
        .build()
}

fn create_header_entry(link: line_types::Link, base_url: &Url) -> ListBoxRow {
    let label = link.label.unwrap_or(link.url.clone());
    let url = validate_url(&link.url, base_url);
    let url_is_invalid = url.is_err();

    let label_widget = Label::builder()
        .label(label)
        .tooltip_text(match url {
            // The function that opens the link when you click a header entry actually relies on
            // this tooltip being the correct url, so make sure that it is either a valid absolute url that
            // actually points to the correct place or that the row is not activatable.
            Ok(url) => url,
            Err(_) => "Broken url".into(),
        })
        .wrap(true)
        .build();

    let row = ListBoxRow::builder()
        .child(&label_widget)
        .activatable(!url_is_invalid)
        .build();
    if url_is_invalid {
        row.add_css_class("dim-label");
    }

    row
}
