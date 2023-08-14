mod imp;

use crate::athn_document::form;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, Adjustment};

glib::wrapper! {
    pub struct FloatFormField(ObjectSubclass<imp::FloatFormField>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl FloatFormField {
    pub fn new(id: form::ID, field: form::FloatField) -> Self {
        let label = field.global.label.unwrap_or(id.id_cloned());
        let min = field.min.unwrap_or(f64::MIN);
        let max = field.max.unwrap_or(f64::MAX);
        let step = field.step.unwrap_or(0.001);
        let default = field.global.default.unwrap_or(0.001);
        let digits = calculate_digits(step).max(calculate_digits(default));

        let widget: Self = Object::builder()
            .property("id", id.id())
            .property("label", label)
            .property("default", default)
            .build();

        let adjustment = Adjustment::new(default, min, max, step, 0., 0.);
        widget.imp().entry.set_adjustment(&adjustment);

        let new_max = widget.imp().closest_tick(&max);
        let new_min = widget.imp().closest_tick(&min);
        widget.imp().entry.set_range(new_min.unwrap_or(min), new_max.unwrap_or(max));

        widget.imp().entry.set_digits(digits);

        widget
    }
}

fn calculate_digits(n: f64) -> u32 {
    if n.fract() < 0.00000001 {
        return 0;
    };
    for i in 1..13 {
        let power = 10u64.pow(i) as f64;
        let fract_iteration = (n.fract() * power).fract();
        let epsilon = 0.000000000001 * power;
        if fract_iteration < epsilon || fract_iteration > 1. - epsilon  {
            return i;
        };
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_0_digits() {
        let n = 1.;

        assert_eq!(0, calculate_digits(n));
    }

    #[test]
    fn calculate_2_digits() {
        let n = 3.44;

        assert_eq!(2, calculate_digits(n));
    }

    #[test]
    fn calculate_10_digits() {
        let n = 1.0123456789;

        assert_eq!(10, calculate_digits(n));
    }
}
