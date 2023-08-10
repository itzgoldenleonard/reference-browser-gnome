use adw::prelude::*;
use adw::subclass::prelude::*;
use email_address::EmailAddress;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, Entry, Label};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/email_form_field.ui")]
#[properties(wrapper_type = super::EmailFormField)]
pub struct EmailFormField {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub label_widget: TemplateChild<Label>,

    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set)]
    label: RefCell<String>,
    #[property(get, set = Self::valid_setter)]
    valid: Cell<bool>,
    #[property(get, set)]
    optional: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for EmailFormField {
    const NAME: &'static str = "AthnEmailFormField";
    type Type = super::EmailFormField;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl EmailFormField {
    #[template_callback]
    fn on_entry_changed(&self, entry: &Entry) {
        let text = &entry.text();
        let obj = self.obj();

        let valid = if text.is_empty() && obj.optional() {
            true
        } else {
            EmailAddress::is_valid(text)
        };
        if obj.valid() != valid {
            obj.set_valid(valid)
        };

        obj.emit_by_name::<()>("updated", &[&obj.id(), &text, &valid]);
    }

    fn valid_setter(&self, valid: bool) {
        if valid {
            self.obj().remove_css_class("error");
        } else {
            self.obj().add_css_class("error");
        }
        self.valid.set(valid);
    }
}

impl ObjectImpl for EmailFormField {
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

        let obj = self.obj();
        obj.bind_property::<Label>("label", self.label_widget.as_ref(), "label")
            .sync_create()
            .build();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("updated")
                .param_types([
                    String::static_type(),
                    String::static_type(),
                    bool::static_type(),
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for EmailFormField {}
impl BoxImpl for EmailFormField {}
