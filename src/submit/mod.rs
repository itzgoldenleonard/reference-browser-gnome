mod imp;

use glib::Object;
use gtk::glib;
use crate::athn_document::form;

glib::wrapper! {
    pub struct SubmitFormField(ObjectSubclass<imp::SubmitFormField>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SubmitFormField {
    pub fn new(id: form::ID, label: Option<String>, destination: String, redirect: bool) -> Self {
        let true_label = match label {
            None => id.id(),
            Some(label) => label,
        };

        Object::builder()
            .property("label", true_label)
            .property("destination", destination)
            .property("redirect", redirect)
            .build()
    }
}
