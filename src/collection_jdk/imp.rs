use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib::{self, Properties}};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::CollectionJdk)]
pub struct CollectionJdk {
    #[property(get, set)]
    pub title: RefCell<String>,
    #[property(get, set)]
    pub tasks: OnceCell<gio::ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionJdk {
    const NAME: &'static str = "JdkeyCollectionJdk";
    type Type = super::CollectionJdk;
}

#[glib::derived_properties]
impl ObjectImpl for CollectionJdk {}