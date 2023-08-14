use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, Label, SpinButton};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/float_form_field.ui")]
#[properties(wrapper_type = super::FloatFormField)]
pub struct FloatFormField {
    #[template_child]
    pub entry: TemplateChild<SpinButton>,
    #[template_child]
    pub label_widget: TemplateChild<Label>,

    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set)]
    label: RefCell<String>,
    #[property(get, set)]
    default: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for FloatFormField {
    const NAME: &'static str = "AthnFloatFormField";
    type Type = super::FloatFormField;
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
impl FloatFormField {
    #[template_callback]
    fn on_entry_changed(&self, entry: &SpinButton) {
        let value = &entry.value();
        let obj = self.obj();

        if let Some(closest_tick) = self.closest_tick(value) {
            self.entry.set_value(closest_tick);
        }

        obj.emit_by_name::<()>("updated", &[&obj.id(), &value, &true]);
    }

    pub fn closest_tick(&self, value: &f64) -> Option<f64> {
        let default = self.obj().default();
        let (step, _) = self.entry.increments();
        let (min, max) = self.entry.range();

        let offset = (value - default) % step;
        if offset == 0. { return None };
        let closest_tick = if offset > step / 2. {
            value - offset + step
        } else {
            value - offset
        };
        if closest_tick > max {
            Some(closest_tick - step)
        } else if closest_tick < min {
            Some(closest_tick + step) 
        } else {
            Some(closest_tick)
        }
    }
}

impl ObjectImpl for FloatFormField {
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
                    f64::static_type(),
                    bool::static_type(),
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for FloatFormField {}
impl BoxImpl for FloatFormField {}
