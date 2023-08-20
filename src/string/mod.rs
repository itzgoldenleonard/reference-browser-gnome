mod imp;

use crate::athn_document::form;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct StringFormField(ObjectSubclass<imp::StringFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl StringFormField {
    pub fn new(form_idx: usize, id: form::ID, field: form::StringField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());
        let default = field.global.default.unwrap_or_default();
        let optional = field.global.optional;

        let min = field.min;
        let label = format!(
            "{label}{}",
            match min {
                None => "".to_string(),
                Some(min) => format!(" (Minimum character count: {})", min.get()),
            }
        );

        let widget: Self = Object::builder()
            .property("form-idx", form_idx as u64)
            .property("id", id.id())
            .property("label", label)
            .property("optional", optional)
            .build();

        if let Some(min) = min {
            widget.set_min_length(min.get());
        }
        if let Some(max) = field.max {
            widget.imp().entry.set_max_length(max.get() as i32);
        }

        widget.imp().entry.set_visibility(!field.secret);
        widget.imp().entry.set_truncate_multiline(!field.multiline);

        widget.imp().entry.set_text(default.as_str());
        widget.set_valid(widget.imp().is_input_valid(&default));

        widget
    }
}
