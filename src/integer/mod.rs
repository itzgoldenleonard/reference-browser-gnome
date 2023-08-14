mod imp;

use crate::athn_document::form;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, Adjustment};

glib::wrapper! {
    pub struct IntFormField(ObjectSubclass<imp::IntFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl IntFormField {
    pub fn new(id: form::ID, field: form::IntField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());
        let min = field.min.unwrap_or(i64::MIN);
        let max = field.max.unwrap_or(i64::MAX);
        let step = field.step.unwrap_or(1);
        let default = field.global.default.unwrap_or(0);

        let widget: Self = Object::builder()
            .property("id", id.id())
            .property("label", label)
            .build();

        let adjustment =
            Adjustment::new(default as f64, min as f64, max as f64, step as f64, 1., 1.);
        widget.imp().entry.set_adjustment(&adjustment);

        widget
    }
}
