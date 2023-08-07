use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
//use glib::subclass::Signal;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, Calendar, CompositeTemplate, SpinButton};
use std::cell::RefCell;

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/date_form_field.ui")]
#[properties(wrapper_type = super::DateFormField)]
pub struct DateFormField {
    #[template_child]
    pub calendar: TemplateChild<Calendar>,
    #[template_child]
    pub hour: TemplateChild<SpinButton>,
    #[template_child]
    pub minute: TemplateChild<SpinButton>,

    #[property(get, set)]
    id: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for DateFormField {
    const NAME: &'static str = "AthnDateFormField";
    type Type = super::DateFormField;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl DateFormField { }

impl ObjectImpl for DateFormField {
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
impl WidgetImpl for DateFormField { }
impl BoxImpl for DateFormField {}
