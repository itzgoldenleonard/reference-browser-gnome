mod imp;

use crate::athn_document::form;
use glib::Object;
use gtk::glib;
use adw::subclass::prelude::*;
use adw::prelude::*;
use std::time::SystemTime;

glib::wrapper! {
    pub struct DateFormField(ObjectSubclass<imp::DateFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DateFormField {
    pub fn new(id: form::ID, field: form::DateField) -> Self {
        let widget: Self = Object::builder()
            .property("id", id.id())
            .build();

        if field.time {
            widget.imp().calendar.set_visible(false);
            widget.imp().hour.set_visible(true);
            widget.imp().minute.set_visible(true);
        }
        if field.date {
            widget.imp().calendar.set_visible(true);
            widget.imp().hour.set_visible(false);
            widget.imp().minute.set_visible(false);
        }

        if let Some(default) = field.global.default {
            if let Ok(default) = convert_default(default) {
                widget.imp().set_datetime(default);
            }
        }

        widget
    }

}

fn convert_default(default: SystemTime) -> Result<glib::DateTime, glib::BoolError> {
    let default = default.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap_or_default();
    glib::DateTime::from_unix_utc(default.as_secs() as i64)
}
