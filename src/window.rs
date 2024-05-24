use adw::subclass::prelude::*;
use gtk::{prelude::*, Label, ListBoxRow};
use gtk::{gio, glib, pango};

use crate::collection_jdk::CollectionJdk;
use crate::jdk_util::{list_all_sdks};

mod imp {
    use std::cell::OnceCell;

    use gtk::ListBox;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/soneca/jdk/import/ssl/keys/window.ui")]
    pub struct JdkImportSslKeysWindow {
        #[template_child]
        pub collections_list: TemplateChild<ListBox>,
        pub collections: OnceCell<gio::ListStore>,
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

    fn setup_collections(&self) {
        let collections = gio::ListStore::new::<CollectionJdk>();
        for sdk in list_all_sdks() {
            self.imp().collections_list.append(&sdk);
        }
    }

    fn setup_callbacks(&self) {
        self.imp().collections_list.connect_row_selected(
            glib::clone!(@weak self as window => move |_, row| {
                // let index = row.index();
                // let selected_collection = window.collections()
                //     .item(index as u32)
                //     .expect("There needs to be an object at this position.")
                //     .downcast::<CollectionObject>()
                //     .expect("The object needs to be a `CollectionObject`.");
                // window.set_current_collection(selected_collection);
                // window.imp().split_view.set_show_content(true);
                println!("teste");
            }),
        );
    }
}
