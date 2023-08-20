use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{clone, ParamSpec, Properties, Value};
use gtk::gio::File;
use gtk::{glib, CompositeTemplate, FileDialog};
use adw::ButtonContent;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/org/athn/browser/gnome/file_form_field.ui")]
#[properties(wrapper_type = super::FileFormField)]
pub struct FileFormField {
    #[template_child]
    pub picker: TemplateChild<FileDialog>,
    #[template_child]
    pub label_widget: TemplateChild<ButtonContent>,

    #[property(get, set)]
    form_idx: Cell<u64>,
    #[property(get, set)]
    id: RefCell<String>,
    #[property(get, set = Self::valid_setter)]
    valid: Cell<bool>,
    #[property(get, set)]
    max_file_size: Cell<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileFormField {
    const NAME: &'static str = "AthnFileFormField";
    type Type = super::FileFormField;
    type ParentType = gtk::Button;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl FileFormField {
    fn valid_setter(&self, valid: bool) {
        if valid {
            self.obj().remove_css_class("error");
        } else {
            self.obj().add_css_class("error");
        }
        self.valid.set(valid);
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
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("updated")
                    .param_types([
                        u64::static_type(),
                        String::static_type(),
                        File::static_type(),
                        bool::static_type(),
                    ])
                    .build(),
                Signal::builder("too-big-file-selected").build(),
            ]
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
                if let Ok(size) = file.query_info("standard::size", gtk::gio::FileQueryInfoFlags::NONE, None::<&gtk::gio::Cancellable>) {
                    let size = size.size();
                    if size > button.obj().max_file_size() as i64 && button.obj().max_file_size() != 0 {
                        return button.obj().emit_by_name::<()>("too-big-file-selected", &[]);
                    }
                };
                button.obj().emit_by_name::<()>("updated", &[&button.obj().id(), &file, &true]);
                button.obj().set_valid(true);
            }
        }));
    }
}
