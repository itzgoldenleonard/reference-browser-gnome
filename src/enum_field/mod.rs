mod imp;

use crate::athn_document::form;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, StringObject};

glib::wrapper! {
    pub struct EnumFormField(ObjectSubclass<imp::EnumFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EnumFormField {
    pub fn new(form_idx: usize, id: form::ID, field: form::StringField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());
        let optional = field.global.optional;

        let widget: Self = Object::builder()
            .property("form-idx", form_idx as u64)
            .property("id", id.id())
            .property("label", label)
            .property("optional", optional)
            .build();

        let string_list = &widget.imp().model;
        let variants = field.variant.unwrap_or_default();
        let many_options = variants.len() >= 5;
        if optional {
            string_list.append("");
        }
        for variant in variants {
            if !variant.is_empty() {
                string_list.append(&variant);
            }
        }

        widget.imp().entry.set_enable_search(many_options);
        widget.imp().select_item(field.global.default);

        widget
    }
}

pub fn extract_string_from_object(input: &glib::Object) -> String {
    input
        .downcast_ref::<StringObject>()
        .map(|s| s.string().to_string())
        .unwrap_or_default()
}
