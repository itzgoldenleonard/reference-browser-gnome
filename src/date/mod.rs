mod imp;

use crate::athn_document::form;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct DateFormField(ObjectSubclass<imp::DateFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DateFormField {
    pub fn new(id: form::ID, field: form::DateField) -> Self {
        Object::builder()
            .property("id", id.id())
            .build()
    }
}
