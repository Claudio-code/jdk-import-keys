mod imp;

use std::borrow::Borrow;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

use crate::jdk_util::Dir;

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
    content: String,
    jdk_path: String,
}

impl KeyData {
    pub fn new(key: String, dir: Dir) -> Self {
        Self {
            jdk_path: dir.path_name(),
            content: key,
        }
    }

    pub fn content(&self) -> String {
        self.content.to_string()
    }

    pub fn jdk_path(&self) -> String {
        self.jdk_path.to_string()
    }
}
