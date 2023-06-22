mod imp;

use crate::athn_document::form;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SubmitFormField(ObjectSubclass<imp::SubmitFormField>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl SubmitFormField {
    /// You are expected to check that destination is a valid absolute URL or None
    pub fn new(
        id: form::ID,
        label: Option<String>,
        destination: Option<String>,
        redirect: bool,
    ) -> Self {
        let label = match label {
            None => id.id(),
            Some(label) => label,
        };
        let invalid_url = destination.is_none();

        Object::builder()
            .property("label", label)
            .property("destination", destination.unwrap_or_default())
            .property("redirect", redirect)
            .property("invalid-url", invalid_url)
            .build()
    }
}
