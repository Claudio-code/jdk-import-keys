use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::once_cell::sync::OnceCell;

use super::KeyData;

#[derive(Properties, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/soneca/jdk/import/ssl/keys/key_object/key_object_row.ui")]
#[properties(wrapper_type = super::KeyObject)]
pub struct KeyObject {
    #[property(get, set, construct_only)]
    title: OnceCell<String>,
    #[property(get, set, construct_only)]
    pub data: OnceCell<KeyData>,
}

#[glib::object_subclass]
impl ObjectSubclass for KeyObject {
    const NAME: &'static str = "JdKeyObject";
    type Type = super::KeyObject;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template()
    }
}

#[glib::derived_properties]
impl ObjectImpl for KeyObject {}
impl WidgetImpl for KeyObject {}
impl BoxImpl for KeyObject {}
impl ListBoxRowImpl for KeyObject {}
impl ActionRowImpl for KeyObject {}
impl PreferencesRowImpl for KeyObject {}
