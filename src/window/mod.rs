mod imp;
mod input;

use crate::athn_document::form;
use crate::athn_document::{line_types, line_types::MainLine, Document, Metadata};
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{ActionRow, Application, ButtonContent, ExpanderRow};
use base64::{engine::general_purpose, Engine as _};
use email_address::EmailAddress;
use gio::File;
use glib::{clone, closure_local, source::PRIORITY_DEFAULT, GString, Object};
use gtk::{
    gio, glib, CheckButton, Label, ListBox, ListBoxRow, Orientation::Horizontal, Separator,
    TextBuffer, TextIter, TextTagTable, TextView,
};
use input::*;
use serde::Deserialize;
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use url::Url;
// Custom widgets
use crate::date::DateFormField;
use crate::email::EmailFormField;
use crate::enum_field::EnumFormField;
use crate::file::FileFormField;
use crate::float::FloatFormField;
use crate::integer::IntFormField;
use crate::string::StringFormField;
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
                TextLine(content) => match self.current_text_block() {
                    None => self
                        .imp()
                        .canvas
                        .append(&(create_text_block(&self.imp().text_block_tag_table, content))),
                    Some(text_block) => append_text_to_block(&text_block, content),
                },
                LinkLine(link) => self.imp().canvas.append(&create_link_line(&self, link, base_url)),
            }
        }
    }

    /// Returns the code block at the end of the canvas, returns 'None' if the last child of the canvas is not a code block
    fn current_code_block(&self) -> Option<TextView> {
        self.imp()
            .canvas
            .last_child()?
            .last_child()
            .and_downcast::<TextView>()
            .filter(|obj| obj.is_monospace())
    }

    /// Returns the text block at the end of the canvas, returns 'None' if the last child of the canvas is not a text block
    fn current_text_block(&self) -> Option<TextView> {
        self.imp()
            .canvas
            .last_child()?
            .last_child()
            .and_downcast::<TextView>()
            .filter(|obj| !obj.is_monospace())
    }

    fn render_main_line(&self, line: MainLine, base_url: &Url) {
        use MainLine::*;
        macro_rules! append {
            ($x:expr) => {
                self.imp().canvas.append(&$x)
            };
        }
        match line {
            TextLine(content) => match self.current_text_block() {
                None => append!(create_text_block(&self.imp().text_block_tag_table, content)),
                Some(text_block) => {
                    text_block
                        .buffer()
                        .insert_at_cursor(format!("\n{}", content).as_str());
                }
            },
            LinkLine(link) => append!(create_link_line(&self, link, base_url)),
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
            FormFieldLine(form_count, line) => {
                self.render_form_field(line, base_url, form_count - 1);
            }
        }
    }

    fn render_form_field(&self, field: form::FormField, base_url: &Url, form_idx: usize) {
        use form::FormField::*;
        macro_rules! append {
            ($x:expr) => {
                self.imp().canvas.append(&$x)
            };
        }
        let mut form_data = self.imp().form_data.borrow_mut();
        if form_data.is_empty() {
            form_data.push(vec![]);
        }
        std::mem::drop(form_data);
        match field {
            Submit(id, field) => append!(create_submit_form_field(
                self, form_idx, id, field, base_url
            )),
            Integer(id, field) => append!(create_int_form_field(self, form_idx, id, field)),
            Float(id, field) => append!(create_float_form_field(self, form_idx, id, field)),
            String(id, field) if field.variant.is_some() => {
                append!(create_enum_form_field(self, form_idx, id, field))
            }
            String(id, field) => append!(create_string_form_field(self, form_idx, id, field)),
            Boolean(id, field) => append!(create_bool_form_field(self, form_idx, id, field)),
            Date(id, field) => append!(create_date_form_field(self, form_idx, id, field)),
            Email(id, field) => append!(create_email_form_field(self, form_idx, id, field)),
            File(id, field) => append!(create_file_form_field(self, form_idx, id, field)),
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

fn create_text_block(tag_table: &TextTagTable, content: String) -> TextView {
    let buffer = TextBuffer::builder().tag_table(&tag_table).build();

    buffer.connect_changed(apply_formatting_tags);
    // It's important that the text is inserted after we've connected to the changed signal,
    // otherwise the callback wont be called for the first line
    buffer.insert_at_cursor(&content);

    let widget = TextView::builder()
        .editable(false)
        .wrap_mode(gtk::WrapMode::WordChar)
        .cursor_visible(false)
        .build();
    widget.set_buffer(Some(&buffer));

    widget
}

fn apply_formatting_tags(buffer: &TextBuffer) {
    use gtk::TextSearchFlags as flags;

    let end_of_buffer = buffer.end_iter();
    let mut start_of_search = buffer.end_iter();
    start_of_search.set_line_offset(0);
    let start_of_last_line = start_of_search;
    while !start_of_search.is_end() {
        let bold_start = start_of_search
            .forward_search("\\b", flags::VISIBLE_ONLY, Some(&end_of_buffer))
            .map(|r| r.0);
        let italic_start = start_of_search
            .forward_search("\\i", flags::VISIBLE_ONLY, Some(&end_of_buffer))
            .map(|r| r.0);
        let preformatted_start = start_of_search
            .forward_search("\\p", flags::VISIBLE_ONLY, Some(&end_of_buffer))
            .map(|r| r.0);
        let formatting_end = start_of_search
            .forward_search("\\r", flags::VISIBLE_ONLY, Some(&end_of_buffer))
            .map(|r| r.1)
            .unwrap_or(end_of_buffer);

        if let Some(bold_start) = bold_start {
            if bold_start.in_range(&start_of_search, &formatting_end) {
                buffer.apply_tag_by_name("bold", &bold_start, &formatting_end);
            }
        }
        if let Some(italic_start) = italic_start {
            if italic_start.in_range(&start_of_search, &formatting_end) {
                buffer.apply_tag_by_name("italic", &italic_start, &formatting_end);
            }
        }
        if let Some(preformatted_start) = preformatted_start {
            if preformatted_start.in_range(&start_of_search, &formatting_end) {
                buffer.apply_tag_by_name("preformatted", &preformatted_start, &formatting_end);
            }
        }
        start_of_search = formatting_end;
    }
    remove_formatting_tags(buffer, &start_of_last_line, &end_of_buffer);
}

fn remove_formatting_tags(buffer: &TextBuffer, start: &TextIter, end: &TextIter) {
    use gtk::TextSearchFlags as flags;
    if let Some((mut tag_start, mut tag_end)) =
        start.forward_search("\\b", flags::VISIBLE_ONLY, Some(end))
    {
        buffer.delete(&mut tag_start, &mut tag_end);
    } else if let Some((mut tag_start, mut tag_end)) =
        start.forward_search("\\i", flags::VISIBLE_ONLY, Some(end))
    {
        buffer.delete(&mut tag_start, &mut tag_end);
    } else if let Some((mut tag_start, mut tag_end)) =
        start.forward_search("\\p", flags::VISIBLE_ONLY, Some(end))
    {
        buffer.delete(&mut tag_start, &mut tag_end);
    } else if let Some((mut tag_start, mut tag_end)) =
        start.forward_search("\\r", flags::VISIBLE_ONLY, Some(end))
    {
        buffer.delete(&mut tag_start, &mut tag_end);
    }
}

fn append_text_to_block(text_view: &TextView, content: String) {
    let buffer = text_view.buffer();
    buffer.insert_at_cursor(format!("\n{}", content).as_str());
}

fn create_int_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::IntField,
) -> IntFormField {
    let default = field.global.default.unwrap_or(0);

    let widget = IntFormField::new(form_idx, id.clone(), field);

    let new_input_data = Input {
        id,
        value: InputTypes::Int(Some(default)),
        valid: true,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_: &IntFormField, form_idx: u64, id: String, value: i32, valid: bool| {
            let id = form::ID::new(&id).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::Int(Some(value.into())), valid);
        }),
    );

    widget
}

fn create_float_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::FloatField,
) -> FloatFormField {
    let default = field.global.default.unwrap_or(0.001);

    let widget = FloatFormField::new(form_idx, id.clone(), field);

    let new_input_data = Input {
        id,
        value: InputTypes::Float(Some(default)),
        valid: true,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_: &FloatFormField, form_idx: u64, id: String, value: f64, valid: bool| {
            let id = form::ID::new(&id).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::Float(Some(value)), valid);
        }),
    );

    widget
}

fn create_string_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::StringField,
) -> StringFormField {
    let default = field.global.default.clone();
    let widget = StringFormField::new(form_idx, id.clone(), field);

    let valid = widget
        .imp()
        .is_input_valid(&default.clone().unwrap_or_default());

    let new_input_data = Input {
        id,
        value: InputTypes::String(default),
        valid,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_form_field: &StringFormField, form_idx: u64, id: String, input: String, valid: bool| {
            let id = form::ID::new(&id).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::String(Some(input)), valid);
        }),
    );

    widget
}

fn create_enum_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::StringField,
) -> EnumFormField {
    let widget = EnumFormField::new(form_idx, id.clone(), field);
    let default = widget.imp().entry.selected_item();
    let default = default.map(|v| crate::enum_field::extract_string_from_object(&v));

    let new_input_data = Input {
        id,
        value: InputTypes::String(default),
        valid: true,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_form_field: &EnumFormField, form_idx: u64, id: String, input: Option<String>, valid: bool| {
            let id = form::ID::new(&id).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::String(input), valid);
        }),
    );

    widget
}

fn create_bool_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::BoolField,
) -> CheckButton {
    let label = field.global.label.unwrap_or(id.id_cloned());
    let optional = field.global.optional;
    let default = match (field.global.default, optional) {
        (None, true) => None,
        (None, false) => Some(false),
        (Some(v), _) => Some(v),
    };

    let widget = CheckButton::with_label(&label);
    widget.set_tooltip_text(Some(&id.id_cloned()));
    widget.set_has_tooltip(false);
    widget.set_widget_name(&form_idx.to_string());
    match default {
        Some(default) => widget.set_active(default),
        None => widget.set_inconsistent(true),
    }

    let new_input_data = Input {
        id,
        value: InputTypes::Bool(default),
        valid: true,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "toggled",
        false,
        closure_local!(@watch window => move |button: &CheckButton| {
            let id = form::ID::new(button.tooltip_text().unwrap().as_str()).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            let form_idx: usize = button.widget_name().parse().unwrap_or_default();
            button.set_inconsistent(false);
            override_element_by_id(&mut all_data[form_idx], id, InputTypes::Bool(Some(button.is_active())), true);
        }),
    );

    widget
}

fn create_date_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::DateField,
) -> DateFormField {
    let default = field.global.default;

    let widget = DateFormField::new(form_idx, id.clone(), field);

    let new_input_data = Input {
        id,
        value: InputTypes::Date(default),
        valid: true,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_form_field: &DateFormField, form_idx: u64, id: String, time: glib::DateTime| {
            let id = form::ID::new(&id).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            let time_formatted = SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(time.to_unix() as u64));
            override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::Date(time_formatted), true);
        }),
    );

    widget
}

fn create_email_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::EmailField,
) -> EmailFormField {
    let default = field.global.default.clone();
    let valid = if default.is_none() && field.global.optional == false {
        false
    } else {
        true
    };
    let widget = EmailFormField::new(form_idx, id.clone(), field);

    let new_input_data = Input {
        id,
        value: InputTypes::Email(default),
        valid,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_form_field: &EmailFormField, form_idx: u64, id: String, email: String, valid: bool| {
            let id = form::ID::new(&id).unwrap();
            let mut all_data = window.imp().form_data.borrow_mut();
            let email_formatted = EmailAddress::from_str(&email).ok();
            override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::Email(email_formatted), valid);
        }),
    );

    widget
}

fn create_file_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::FileField,
) -> FileFormField {
    let optional = field.global.optional;
    let widget = FileFormField::new(form_idx, id.clone(), field);

    let new_input_data = Input {
        id,
        value: InputTypes::File(None),
        valid: optional,
    };
    window.imp().form_data.borrow_mut()[form_idx].push(new_input_data);

    #[allow(unused_must_use)]
    widget.connect_closure(
        "updated",
        false,
        closure_local!(@watch window => move |_form_field: &FileFormField, form_idx: u64, id: String, file: File, valid: bool| {
            let ctx = glib::MainContext::default();
            ctx.spawn_local(clone!(@weak file, @strong id, @weak window, @strong valid => async move {
                let id = form::ID::new(&id).unwrap();
                let mut all_data = window.imp().form_data.borrow_mut();
                let encoded = match base64_encode_file(file).await {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                override_element_by_id(&mut all_data[form_idx as usize], id, InputTypes::File(encoded), valid);
            }));
        }),
    );

    widget.connect_closure(
        "too-big-file-selected",
        false,
        closure_local!(@watch window => move |field: FileFormField| {
            let message = format!("File selected is too big, max size allowed is: {}B", field.max_file_size());
            eprintln!("{message}");

            let toast = adw::Toast::new(message.as_str());
            window.imp().toaster.add_toast(toast);
            if let Some(toast_widget) = window.imp().toaster.last_child() {
                toast_widget.add_css_class("error");
            }
        }),
    );

    widget
}

async fn base64_encode_file(file: File) -> Result<String, glib::Error> {
    let reader = file.read_future(PRIORITY_DEFAULT).await?;
    let bytes = reader
        .read_bytes_future(std::i32::MAX as usize, PRIORITY_DEFAULT)
        .await?;
    Ok(general_purpose::STANDARD.encode(bytes))
}

/// Returns an error if the id doesnt exist
fn override_element_by_id(
    vector: &mut Vec<Input>,
    id: form::ID,
    new_value: InputTypes,
    valid: bool,
) -> Result<(), ()> {
    let idx = vector.iter().enumerate().find(|&x| *x.1 == id).ok_or(())?.0;
    vector[idx].value = new_value;
    vector[idx].valid = valid;
    Ok(())
}

fn create_submit_form_field(
    window: &Window,
    form_idx: usize,
    id: form::ID,
    field: form::SubmitField,
    base_url: &Url,
) -> SubmitFormField {
    let url = validate_url(&field.destination, base_url).ok();
    let language_string = window
        .imp()
        .settings
        .borrow()
        .clone()
        .map(|settings| settings.string("language-preference"))
        .unwrap_or_default();

    let widget = SubmitFormField::new(
        form_idx,
        id,
        field.label,
        url,
        field.redirect,
        language_string.to_string(),
    );

    widget.connect_closure(
        "data-request",
        false,
        closure_local!(@watch window => move |button: SubmitFormField| {
            let form_idx = button.form_idx() as usize;
            button.set_invalid_form(!window.imp().is_form_valid(form_idx));
            button.set_serialized_data(serde_json::to_string(&window.imp().form_data.borrow()[form_idx]).unwrap());
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

    widget.connect_closure(
        "server_validation-error",
        false,
        closure_local!(@watch window => move |_button: SubmitFormField, validation_error: std::string::String| {
            let message = "The server responded with the following error";
            eprintln!("{message}:");
            eprintln!("{validation_error}");

            let toast = adw::Toast::new(message);
            toast.set_timeout(0);
            window.imp().toaster.add_toast(toast);
            if let Some(toast_widget) = window.imp().toaster.last_child() {
                toast_widget.add_css_class("error");
            }

            window.imp().server_error_window.set_visible(true);
            let buffer = &window.imp().server_error_buffer;

            let errors: serde_json::Result<Vec<FormValidation>> = serde_json::from_str(&validation_error);
            buffer.set_text(&match errors {
                Ok(errors) => errors.iter().map(|error| format!("Field {}: {}\n", error.id, error.message)).collect(),
                Err(_) => "The browser was unable to understand the error message from the server".to_string(),
            });
        }),
    );

    window.imp().form_data.borrow_mut().push(vec![]);

    widget
}

#[derive(Deserialize)]
struct FormValidation {
    pub id: String,
    pub message: String,
    #[allow(dead_code)]
    pub idx: Option<usize>,
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

fn validate_url(url: &String, base_url: &Url) -> Result<String, url::ParseError> {
    let url_may_be_relative = Url::parse(url);
    if url_may_be_relative == Err(url::ParseError::RelativeUrlWithoutBase) {
        base_url.join(url).map(|u| u.into())
    } else {
        url_may_be_relative.map(|u| u.into())
    }
}

fn create_link_line(window: &Window, link: line_types::Link, base_url: &Url) -> Label {
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

    widget.connect_activate_link(clone!(@weak window => @default-return glib::signal::Inhibit(true), move |_: &Label, uri: &str| {
        window.set_uri(uri);
        glib::signal::Inhibit(true)
    }));

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
            Note => "question-symbolic",
            Warning => "warning-symbolic",
            Danger => "junk-symbolic",
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
