use gtk::glib;
use adw::prelude::*;
use gtk::subclass::prelude::*;
use glib::{ParamSpec, Properties, Value};
use std::cell::RefCell;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::SubmitField)]
pub struct SubmitField {
    #[property(get, set = Self::label_property_setter)]
    pub label_property: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for SubmitField {
    const NAME: &'static str = "AthnSubmitField";
    type Type = super::SubmitField;
    type ParentType = gtk::Button;
}

impl SubmitField {
    fn label_property_setter(&self, input: String) {
        self.obj().set_label(&input);
        *self.label_property.borrow_mut() = input;
    }
}

impl ObjectImpl for SubmitField {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for SubmitField {}
impl ButtonImpl for SubmitField {}
