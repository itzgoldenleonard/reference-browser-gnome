use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::DateTime;
use glib::{ParamSpec, Properties, Value};
use gtk::{glib, CompositeTemplate, Button, Label};
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/file_form_field.ui")]
#[properties(wrapper_type = super::FileFormField)]
pub struct FileFormField {
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub label_widget: TemplateChild<Label>,

    #[property(get, set)]
    label: RefCell<String>,
    #[property(get, set)]
    id: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileFormField {
    const NAME: &'static str = "AthnFileFormField";
    type Type = super::FileFormField;
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
impl FileFormField {
    #[template_callback]
    fn on_button_click(&self, button: &Button) {
        let file_chooser = gtk::FileDialog::new();
        file_chooser.open(None::<&gtk::Window>, None::<&gtk::gio::Cancellable>, |_| {println!("Opened the dialog")});
    }
}

impl ObjectImpl for FileFormField {
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
                .param_types([String::static_type(), DateTime::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for FileFormField {}
impl BoxImpl for FileFormField {}
