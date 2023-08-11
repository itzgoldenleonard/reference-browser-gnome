use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{clone, ParamSpec, Properties, Value};
use gtk::gio::File;
use gtk::{glib, CompositeTemplate, FileDialog};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/file_form_field.ui")]
#[properties(wrapper_type = super::FileFormField)]
pub struct FileFormField {
    #[template_child]
    pub picker: TemplateChild<FileDialog>,

    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set)]
    optional: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileFormField {
    const NAME: &'static str = "AthnFileFormField";
    type Type = super::FileFormField;
    type ParentType = gtk::Button;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl FileFormField {}

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
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("updated")
                .param_types([
                    String::static_type(),
                    File::static_type(),
                    bool::static_type(),
                ])
                .build()]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for FileFormField {}
impl ButtonImpl for FileFormField {
    fn clicked(&self) {
        let ctx = glib::MainContext::default();
        ctx.spawn_local(clone!(@weak self as button => async move {
            if let Ok(file) = button.picker.open_future(None::<&gtk::Window>).await {
                button.obj().emit_by_name::<()>("updated", &[&button.obj().id(), &file, &true]);
            }
        }));
    }
}
