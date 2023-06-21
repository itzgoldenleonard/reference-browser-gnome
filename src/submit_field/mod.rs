mod imp;

use glib::Object;
use gtk::glib;
use crate::athn_document::form;

glib::wrapper! {
    pub struct SubmitField(ObjectSubclass<imp::SubmitField>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SubmitField {
    pub fn new(_id: form::ID, _field: form::SubmitField) -> Self {
        Object::builder()
            //.property("label_property", field.label)
            .build()
    }
}
