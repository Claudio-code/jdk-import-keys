use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, TemplateChild};

use super::CollectionJdkData;

#[derive(glib::Properties, Default, CompositeTemplate)]
#[template(resource = "/org/soneca/jdk/import/ssl/keys/collection_jdk_row.ui")]
#[properties(wrapper_type = super::CollectionJdk)]
pub struct CollectionJdk {
    #[property(get, set, construct_only)]
    title: OnceCell<String>,
    #[property(get, set, construct_only)]
    pub data: OnceCell<CollectionJdkData>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionJdk {
    const NAME: &'static str = "JdkeyCollectionJdk";
    type Type = super::CollectionJdk;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template()
    }
}

#[glib::derived_properties]
impl ObjectImpl for CollectionJdk {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj().clone();
        let tooltip_text = format!("{} - {}", obj.data().package_manager, obj.data().path);
        obj.set_subtitle(&obj.data().package_manager);
        obj.set_tooltip_text(Some(tooltip_text.as_str()));
        obj.set_focus_on_click(true);

        obj.connect_destroy(|item| {
            println!("teqdwq")
        });
    }
}

impl WidgetImpl for CollectionJdk {}
impl BoxImpl for CollectionJdk {}
impl ListBoxRowImpl for CollectionJdk {}
impl ActionRowImpl for CollectionJdk {}
impl PreferencesRowImpl for CollectionJdk {}
