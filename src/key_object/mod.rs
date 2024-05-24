mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct KeyObject(ObjectSubclass<imp::KeyObject>);
}

impl KeyObject {
    pub fn new(content: String) -> Self {
        Object::builder()
            .property("content", content)
            .build()
    }

    pub fn task_data(&self) -> KeyData {
        self.imp().data.borrow().clone()
    }

    pub fn from_task_data(task_data: KeyData) -> Self {
        Self::new(task_data.content)
    }
}

#[derive(Default, Clone, Debug, PartialEq, glib::Boxed)]
#[boxed_type(name = "KeyData")]
pub struct KeyData {
    pub content: String,
    pub jdk_path: String,
}
