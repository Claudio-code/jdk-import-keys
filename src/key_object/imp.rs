use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::KeyData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::KeyObject)]
pub struct KeyObject {
    #[property(name = "content", get, set, type = String, member = content)]
    pub data: RefCell<KeyData>,
}

#[glib::object_subclass]
impl ObjectSubclass for KeyObject {
    const NAME: &'static str = "JdKeyObject";
    type Type = super::KeyObject;
}

#[glib::derived_properties]
impl ObjectImpl for KeyObject {}