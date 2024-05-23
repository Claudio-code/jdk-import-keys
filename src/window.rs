/* window.rs
 *
 * Copyright 2024 soneca
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

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
            collections.append(&sdk);
        }

        // self.imp()
        //     .collections
        //     .set(collections.clone())
        //     .expect("Could not set collections");

        let window_clone = self.clone();
        self.imp().collections_list.bind_model(
            Some(&collections),
            glib::clone!(@weak window_clone => @default-panic, move |obj| {
                let collection_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `CollectionObject`.");
                let row = window_clone.create_collection_row(collection_object);
                row.upcast()
            }),
        )
    }

    fn create_collection_row(
        &self,
        collection_object: &CollectionJdk,
    ) -> ListBoxRow {
        let label = Label::builder()
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();

        collection_object
            .bind_property("title", &label, "label")
            .sync_create()
            .build();

        ListBoxRow::builder().child(&label).build()
    }
}
