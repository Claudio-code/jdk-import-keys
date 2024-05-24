use adw::subclass::prelude::*;
use gtk::{gio, glib, pango};
use gtk::{prelude::*, Label, ListBoxRow};

use crate::collection_jdk::{CollectionJdk, CollectionJdkData};
use crate::jdk_util::list_all_sdks;
use crate::key_object::KeyObject;

mod imp {
    use std::cell::{OnceCell, RefCell};

    use adw::NavigationSplitView;
    use gtk::{ListBox, Stack};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/soneca/jdk/import/ssl/keys/window.ui")]
    pub struct JdkImportSslKeysWindow {
        #[template_child]
        pub collections_list: TemplateChild<ListBox>,
        #[template_child]
        pub keys_list: TemplateChild<ListBox>,
        #[template_child]
        pub split_view: TemplateChild<NavigationSplitView>,
        #[template_child]
        pub stack: TemplateChild<Stack>,
        pub collections: OnceCell<gio::ListStore>,
        pub current_collection: RefCell<Option<CollectionJdk>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for JdkImportSslKeysWindow {
        const NAME: &'static str = "JdkImportSslKeysWindow";
        type Type = super::JdkImportSslKeysWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for JdkImportSslKeysWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_collections();
            obj.setup_callbacks();
        }
    }

    impl WidgetImpl for JdkImportSslKeysWindow {}
    impl WindowImpl for JdkImportSslKeysWindow {}
    impl ApplicationWindowImpl for JdkImportSslKeysWindow {}
    impl AdwApplicationWindowImpl for JdkImportSslKeysWindow {}
}

glib::wrapper! {
    pub struct JdkImportSslKeysWindow(ObjectSubclass<imp::JdkImportSslKeysWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl JdkImportSslKeysWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn collections(&self) -> gio::ListStore {
        self.imp()
            .collections
            .get()
            .expect("`collections` should be set in `setup_collections`.")
            .clone()
    }

    fn setup_collections(&self) {
        let collections = gio::ListStore::new::<CollectionJdk>();
        list_all_sdks().into_iter().for_each(|sdk| {
            collections.append(&sdk);
            self.imp().collections_list.append(&sdk);
        });
        self.imp()
            .collections
            .set(collections.clone())
            .expect("Could not set collections");
    }

    fn setup_callbacks(&self) {
        self.set_stack();
        self.collections().connect_items_changed(
            glib::clone!(@weak self as window => move |_, _, _, _| window.set_stack()),
        );

        self.imp().collections_list.connect_row_selected(
            glib::clone!(@weak self as window => move |_, row| {
                let index = row.unwrap().index();
                let selected_collection = window.collections()
                    .item(index as u32)
                    .expect("There needs to be an object at this position.")
                    .downcast::<CollectionJdk>()
                    .expect("The object needs to be a `CollectionJdk`.");
                window.set_current_collection(selected_collection);
                window.imp().split_view.set_show_content(true);
            }),
        );
    }

    fn set_current_collection(&self, collection: CollectionJdk) {
        self.imp().keys_list.remove_all();
        collection
            .data()
            .keys
            .into_iter()
            .for_each(|key_data| self.imp().keys_list.append(&KeyObject::new(key_data)));
        self.imp().keys_list.set_visible(true);
    }

    fn set_stack(&self) {
        if self.collections().n_items() > 0 {
            self.imp().stack.set_visible_child_name("main");
            return;
        }
        self.imp().stack.set_visible_child_name("placeholder");
    }
}
