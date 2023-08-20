use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{clone, ParamSpec, Properties, Value};
use gtk::glib;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Default, Properties)]
#[properties(wrapper_type = super::SubmitFormField)]
pub struct SubmitFormField {
    #[property(get, set)]
    pub form_idx: Cell<u64>,
    #[property(get, set)]
    pub serialized_data: RefCell<String>,
    #[property(get, set)]
    pub destination: RefCell<String>,
    #[property(get, set)]
    pub redirect: Cell<bool>,
    #[property(get, set = Self::invalid_url_setter)]
    pub invalid_url: Cell<bool>,
    #[property(get, set)]
    pub invalid_form: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for SubmitFormField {
    const NAME: &'static str = "AthnSubmitFormField";
    type Type = super::SubmitFormField;
    type ParentType = gtk::Button;
}

impl SubmitFormField {
    fn invalid_url_setter(&self, input: bool) {
        self.obj().set_can_target(!input);
        self.obj().set_focusable(!input);
        if input {
            self.obj().add_css_class("dim-label");
            self.obj()
                .set_label("Invalid destination URL, cannot submit form");
        } else {
            self.obj().remove_css_class("dim-label");
        };

        self.invalid_url.set(input);
    }
}

impl ObjectImpl for SubmitFormField {
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
                Signal::builder("submit-success")
                    .param_types([String::static_type()])
                    .build(),
                Signal::builder("submit-error")
                    .param_types([String::static_type()])
                    .build(),
                Signal::builder("data-request")
                    //.param_types([String::static_type()])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }
}
impl WidgetImpl for SubmitFormField {}
impl ButtonImpl for SubmitFormField {
    fn clicked(&self) {
        self.obj().emit_by_name::<()>("data-request", &[]);
        if self.obj().invalid_form() {
            return self.obj().emit_by_name::<()>(
                "submit-error",
                &[&"A form field in this form is invalid".to_string()],
            );
        }

        /*
        let response = match post(self.obj().destination(), self.obj().serialized_data()) {
            Ok(val) => val,
            Err(e) => {
                return self
                    .obj()
                    .emit_by_name::<()>("submit-error", &[&e.to_string()]);
            }
        };

        self.obj()
            .emit_by_name::<()>("submit-success", &[&response]);
        */

        let ctx = glib::MainContext::default();
        ctx.spawn_local(clone!(@weak self as button => async move {
            let response = match post(button.obj().destination(), button.obj().serialized_data()) {
                Ok(val) => val,
                Err(e) => {
                    return button
                        .obj()
                        .emit_by_name::<()>("submit-error", &[&e.to_string()]);
                }
            };

            button.obj()
                .emit_by_name::<()>("submit-success", &[&response]);
        }));
    }
}

#[tokio::main]
async fn post(destination: String, body: String) -> reqwest::Result<String> {
    let https_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let response = https_client.post(destination).body(body).send().await?;
    let response = response.error_for_status()?;
    response.text().await
}
