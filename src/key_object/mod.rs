mod imp;

use std::borrow::Borrow;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct KeyObject(ObjectSubclass<imp::KeyObject>)
        @extends gtk::Widget, adw::ActionRow, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl KeyObject {
    pub fn new(key_data: KeyData) -> Self {
        Object::builder()
            .property("title", key_data.content.clone())
            .property("data", key_data)
            .build()
    }
}

#[derive(Default, Clone, Debug, PartialEq, glib::Boxed)]
#[boxed_type(name = "KeyData")]
pub struct KeyData {
    pub content: String,
    pub jdk_path: String,
}
