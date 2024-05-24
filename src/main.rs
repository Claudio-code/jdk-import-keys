mod application;
mod config;
mod window;
mod collection_jdk;
mod key_object;
mod jdk_util;

use self::application::JdkImportSslKeysApplication;
use self::window::JdkImportSslKeysWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/jdk-import-ssl-keys.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Load icons resources
    let icons_resources = gio::Resource::load(PKGDATADIR.to_owned() + "/icons.gresource")
        .expect("Could not load icons resources");
    gio::resources_register(&icons_resources);

    let app = JdkImportSslKeysApplication::new("org.soneca.jdk.import.ssl.keys", &gio::ApplicationFlags::empty());
    app.run()
}
