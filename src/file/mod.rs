mod imp;

use crate::athn_document::form;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, FileFilter};

glib::wrapper! {
    pub struct FileFormField(ObjectSubclass<imp::FileFormField>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl FileFormField {
    pub fn new(form_idx: usize, id: form::ID, field: form::FileField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());

        let widget: Self = Object::builder()
            .property("form-idx", form_idx as u64)
            .property("id", id.id())
            .property("valid", field.global.optional)
            .build();

        widget.imp().label_widget.set_label(&label);

        if let Some(max) = field.max {
            widget.set_max_file_size(max.get());
        }

        if let Some(mime_types) = field.allowed_types {
            let filter = FileFilter::new();
            for type_ in mime_types {
                filter.add_mime_type(&type_);
            }
            let model = gtk::gio::ListStore::new(FileFilter::static_type());
            model.append(&filter);
            widget.imp().picker.set_filters(&model);
        } 

        widget.imp().picker.set_title(&label);

        widget
    }
}
