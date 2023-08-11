mod imp;

use crate::athn_document::form;
/*
use adw::prelude::*;
use adw::subclass::prelude::*;
*/
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct FileFormField(ObjectSubclass<imp::FileFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl FileFormField {
    pub fn new(id: form::ID, field: form::FileField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());

        let widget: Self = Object::builder()
            .property("id", id.id())
            .property("label", label)
            .build();

        widget
    }
}
