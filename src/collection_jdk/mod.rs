mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{gio, glib};
use serde::{Deserialize, Serialize};

use crate::key_object::KeyData;
use crate::key_object::KeyObject;

glib::wrapper! {
    pub struct CollectionJdk(ObjectSubclass<imp::CollectionJdk>)
        @extends gtk::Widget, adw::ActionRow, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CollectionJdk {
    pub fn new(jdk_data: CollectionJdkData) -> Self {
        Object::builder()
            .property("data", jdk_data.clone())
            .property("title", jdk_data.title)
            // .property("tasks", tasks)
            .build()
    }

    // pub fn to_collection_data(&self) -> CollectionData {
    //     let title = self.imp().title.borrow().clone();
    //     let tasks_data = self.tasks()
    //         .iter::<KeyObject>()
    //         .filter_map(Result::ok)
    //         .map(|task_object| task_object.task_data())
    //         .collect();
    //     CollectionData { title, tasks_data }
    // }

    // pub fn from_collection_data(collection_data: CollectionData) -> Self {
    //     let title = collection_data.title;
    //     let tasks_to_extend: Vec<KeyObject> = collection_data
    //         .tasks_data
    //         .into_iter()
    //         .map(KeyObject::from_task_data)
    //         .collect();

    //     let tasks = gio::ListStore::new::<KeyObject>();
    //     tasks.extend_from_slice(&tasks_to_extend);
    //     Self::new(&title)
    // }
}

#[derive(Debug, Clone, PartialEq, glib::Boxed)]
#[boxed_type(name = "CollectionJdkData")]
pub struct CollectionJdkData {
    pub title: String,
    pub package_manager: String,
    pub path: String,
    pub keys: Vec<KeyData>,
}
