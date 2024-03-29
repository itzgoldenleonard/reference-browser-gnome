mod imp;

use crate::athn_document::form;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct EmailFormField(ObjectSubclass<imp::EmailFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EmailFormField {
    pub fn new(form_idx: usize, id: form::ID, field: form::EmailField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());
        let optional = field.global.optional;

        let widget: Self = Object::builder()
            .property("form-idx", form_idx as u64)
            .property("id", id.id())
            .property("label", label)
            .property("optional", optional)
            .build();

        if let Some(default) = field.global.default {
            widget.imp().entry.set_text(default.as_str());
        } else {
            widget.set_valid(optional);
        }

        widget
    }
}
