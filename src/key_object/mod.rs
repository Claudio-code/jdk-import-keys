mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct KeyObject(ObjectSubclass<imp::KeyObject>);
}

impl KeyObject {
    pub fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }

    pub fn is_completed(&self) -> bool {
        self.imp().data.borrow().completed
    }

    pub fn task_data(&self) -> KeyData {
        self.imp().data.borrow().clone()
    }

    pub fn from_task_data(task_data: KeyData) -> Self {
        Self::new(task_data.completed, task_data.content)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct KeyData {
    pub completed: bool,
    pub content: String,
}
