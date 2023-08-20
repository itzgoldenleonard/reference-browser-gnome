use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, Label, SpinButton};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/int_form_field.ui")]
#[properties(wrapper_type = super::IntFormField)]
pub struct IntFormField {
    #[template_child]
    pub entry: TemplateChild<SpinButton>,
    #[template_child]
    pub label_widget: TemplateChild<Label>,

    #[property(get, set)]
    form_idx: Cell<u64>,
    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set)]
    label: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for IntFormField {
    const NAME: &'static str = "AthnIntFormField";
    type Type = super::IntFormField;
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
impl IntFormField {
    #[template_callback]
    fn on_entry_changed(&self, entry: &SpinButton) {
        let value = &entry.value_as_int();
        let obj = self.obj();

        obj.emit_by_name::<()>("updated", &[&obj.form_idx(), &obj.id(), &value, &true]);
    }
}

impl ObjectImpl for IntFormField {
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
                    u64::static_type(),
                    String::static_type(),
                    i32::static_type(),
                    bool::static_type(),
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for IntFormField {}
impl BoxImpl for IntFormField {}
