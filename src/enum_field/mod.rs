mod imp;

use crate::athn_document::form;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct EnumFormField(ObjectSubclass<imp::EnumFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EnumFormField {
    pub fn new(id: form::ID, field: form::StringField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());
        let optional = field.global.optional;

        let widget: Self = Object::builder()
            .property("id", id.id())
            .property("label", label)
            .property("optional", optional)
            .build();

        let string_list = &widget.imp().model;
        let variants = field.variant.unwrap_or_default();
        let many_options = variants.len() >= 5;
        for variant in variants {
            string_list.append(&variant);
        }

        widget.imp().entry.set_enable_search(many_options);

        widget
    }
}
