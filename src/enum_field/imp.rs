use super::extract_string_from_object;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, DropDown, Label, StringList};
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
    form_idx: Cell<u64>,
    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set)]
    label: RefCell<String>,
    #[property(get, set)]
    optional: Cell<bool>,
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
        let selected_item = selected_item.map(|item| extract_string_from_object(&item));
        let obj = self.obj();

        obj.emit_by_name::<()>("updated", &[&obj.form_idx(), &obj.id(), &selected_item, &true]);
        if selected_item.is_some_and(|e| e.is_empty()) {
            entry.set_selected(u32::MAX);
        }
    }

    /// Selects inputted item in the dropdown if it's an option
    /// if item is not an option it will select None
    pub fn select_item(&self, item: Option<String>) {
        let options = self.model.snapshot();
        let item_idx = options
            .iter()
            .enumerate()
            .find(|e| extract_string_from_object(e.1) == item.clone().unwrap_or_default())
            .map(|r| r.0 as u32);

        let optional = self.obj().optional();
        self.entry
            .set_selected(item_idx.unwrap_or(if optional { u32::MAX } else { 0 }));
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
                    u64::static_type(),
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
