use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, DropDown, Label, StringObject, StringList};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/enum_form_field.ui")]
#[properties(wrapper_type = super::EnumFormField)]
pub struct EnumFormField {
    #[template_child]
    pub entry: TemplateChild<DropDown>,
    #[template_child]
    pub label_widget: TemplateChild<Label>,
    #[template_child]
    pub model: TemplateChild<StringList>,

    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set)]
    label: RefCell<String>,
    #[property(get, set = Self::valid_setter)]
    valid: Cell<bool>,
    #[property(get, set)]
    optional: Cell<bool>,
    #[property(get, set)]
    min_length: Cell<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for EnumFormField {
    const NAME: &'static str = "AthnEnumFormField";
    type Type = super::EnumFormField;
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
impl EnumFormField {
    #[template_callback]
    fn on_entry_changed(&self, _pspec: &glib::ParamSpec, entry: &DropDown) {
        let selected_item = entry.selected_item();
        let selected_item = selected_item.map(|i| i.downcast_ref::<StringObject>().map(|s| s.string().to_string()).unwrap_or_default());
        let obj = self.obj();

        obj.emit_by_name::<()>("updated", &[&obj.id(), &selected_item, &true]);
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

impl ObjectImpl for EnumFormField {
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
                    Option::<String>::static_type(),
                    bool::static_type(),
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for EnumFormField {}
impl BoxImpl for EnumFormField {}
