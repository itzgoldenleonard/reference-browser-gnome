use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::DateTime;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, Calendar, CompositeTemplate, SpinButton, Label};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

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
impl ObjectSubclass for DateFormField {
    const NAME: &'static str = "AthnDateFormField";
    type Type = super::DateFormField;
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
impl DateFormField {
    #[template_callback]
    fn on_minute_wrapped(&self, spinner: &SpinButton) {
        let hour = &self.hour;
        let hour_value = hour.value();
        let wrap_up = spinner.value_as_int() == 0;

        hour.set_value(if wrap_up {
            hour_value + 1.
        } else {
            hour_value - 1.
        });
    }

    #[template_callback]
    fn on_day_selected(&self, _calendar: &Calendar) {
        if let Ok(time) = self.get_time() {
            self.obj()
                .emit_by_name::<()>("updated", &[&self.obj().form_idx(), &self.obj().id(), &time]);
        }
    }

    #[template_callback]
    fn on_time_change(&self, _spinner: &SpinButton) {
        if (self.hour.value_as_int() == 23 && self.minute.value_as_int() == 59)
            || (self.hour.value_as_int() == 0 && self.minute.value_as_int() == 0)
        {
            self.minute.set_wrap(false);
        } else {
            self.minute.set_wrap(true);
        }

        if let Ok(time) = self.get_time() {
            self.obj()
                .emit_by_name::<()>("updated", &[&self.obj().form_idx(), &self.obj().id(), &time]);
        }
    }

    /// This function returns the time in UTC
    fn get_time(&self) -> Result<DateTime, glib::BoolError> {
        let date = DateTime::from_local(
            self.calendar.year(),
            self.calendar.month() + 1,
            self.calendar.day(),
            self.hour.value_as_int(),
            self.minute.value_as_int(),
            0.,
        )?;
        date.to_utc()
    }

    pub fn set_datetime(&self, datetime: DateTime) {
        if let Ok(datetime) = datetime.to_local() {
            self.calendar.select_day(&datetime);
            self.hour.set_value(datetime.hour().into());
            self.minute.set_value(datetime.minute().into());
        }
    }
}

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

        let obj = self.obj();
        obj.bind_property::<Label>("label", self.label_widget.as_ref(), "label")
            .sync_create()
            .build();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("updated")
                .param_types([u64::static_type(), String::static_type(), DateTime::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for DateFormField {}
impl BoxImpl for DateFormField {}
