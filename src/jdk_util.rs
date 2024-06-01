use crate::collection_jdk::{CollectionJdk, CollectionJdkData};
use crate::key_object::KeyData;
use adw::subclass::prelude::*;
use clap::Parser;
use gtk::glib;
use std::fmt::Debug;
use std::fs::{DirEntry, ReadDir};
use std::io::{Error, ErrorKind, Read};
use std::process::{Child, ExitStatus};
use std::str::FromStr;
use std::{fs, path::PathBuf, process::Command};
use strum_macros::EnumString;

const ASDF_JAVA_PATH: &str = "/.asdf/installs/java";
const JETBRAINS_JAVA_PATH: &str = "/.jdks";
const SDKMAN_JAVA_PATH: &str = "/.sdkman/candidates/java";
const KEYTOOL_PATH: &str =
    "{}/bin/keytool -list -v -keystore {}/lib/security/cacerts -storepass changeit | grep Alias";

#[derive(Debug, Clone)]
pub struct Dir {
    path_name: String,
    is_dir: bool,
}

impl Dir {
    pub fn new(dir: DirEntry) -> Self {
        Self {
            is_dir: dir.path().is_dir(),
            path_name: dir.path().to_str().unwrap().to_string(),
        }
    }

    pub fn jdk_name(&self) -> &str {
        self.path_name.split("/").last().unwrap()
    }

    pub fn path_name(&self) -> String {
        self.path_name.clone()
    }

    pub fn is_valid_dir(&self) -> bool {
        !self.path_name.contains("current") && self.is_dir
    }
}

pub fn list_all_sdks() -> Vec<CollectionJdk> {
    let mut all_sdks: Vec<CollectionJdk> = Vec::new();
    all_sdks.append(&mut load_jdks_sdkman());
    all_sdks.append(&mut load_jdks_jetbrains());
    all_sdks.append(&mut load_jdks_asdf());
    all_sdks
}

fn load_jdks_asdf() -> Vec<CollectionJdk> {
    if dirs::home_dir().is_none() {
        return Vec::new();
    }
    let binding = dirs::home_dir().unwrap();
    let mut home_dir = binding.display().to_string();
    home_dir.push_str(ASDF_JAVA_PATH.into());
    if let Ok(paths) = fs::read_dir(home_dir) {
        return list_jdks(paths, "Asdf");
    }
    return Vec::new();
}

fn load_jdks_jetbrains() -> Vec<CollectionJdk> {
    if dirs::home_dir().is_none() {
        return Vec::new();
    }
    let binding = dirs::home_dir().unwrap();
    let mut home_dir = binding.display().to_string();
    home_dir.push_str(JETBRAINS_JAVA_PATH.into());
    if let Ok(paths) = fs::read_dir(home_dir) {
        return list_jdks(paths, "Jetbrains");
    }
    return Vec::new();
}

fn load_jdks_sdkman() -> Vec<CollectionJdk> {
    if dirs::home_dir().is_none() {
        return Vec::new();
    }
    let binding = dirs::home_dir().unwrap();
    let mut home_dir = binding.display().to_string();
    home_dir.push_str(SDKMAN_JAVA_PATH.into());
    if let Ok(paths) = fs::read_dir(home_dir) {
        return list_jdks(paths, "Sdkman");
    }
    return Vec::new();
}

fn list_jdks(paths: ReadDir, package_manager: &str) -> Vec<CollectionJdk> {
    paths
        .into_iter()
        .map(Result::unwrap)
        .map(Dir::new)
        .filter(Dir::is_valid_dir)
        .map(move |dir| {
            let keys: Vec<KeyData> = list_certs_jdk(dir.path_name())
                .into_iter()
                .map(|key| KeyData::new(key, dir.clone()))
                .collect();
            let collection_jdk_data = CollectionJdkData {
                title: format!("Java-{}", dir.jdk_name()),
                package_manager: package_manager.to_string(),
                path: dir.path_name,
                keys: keys.clone(),
            };
            return CollectionJdk::new(collection_jdk_data);
        })
        .collect()
}

fn list_certs_jdk(jdk_home_dir: String) -> Vec<String> {
    let command = format!("{}/bin/keytool -list -v -keystore {}/lib/security/cacerts -storepass changeit | grep Alias", jdk_home_dir, jdk_home_dir);
    let out = keytool_capture(command).unwrap();

    out.lines()
        .into_iter()
        .map(move |line| line.replace("Alias name: ", "").replace(" [jdk]", ""))
        .collect()
}

fn spawn_child(command: String) -> std::io::Result<Child> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
}

fn keytool_capture(command: String) -> std::io::Result<String> {
    let mut child = spawn_child(command)?;
    match child.wait()?.code() {
        Some(code) if code == 0 || code == 1 || code == 3 => {}
        Some(code) if code == 4 => {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                "Missing Priviledges or Unit not found",
            ))
        }
        // unknown errorcodes
        Some(code) => {
            return Err(Error::new(
                // TODO: Maybe a better ErrorKind, none really seem to fit
                ErrorKind::Other,
                format!("Process exited with code: {code}"),
            ));
        }
        None => {
            return Err(Error::new(
                ErrorKind::Interrupted,
                "Process terminated by signal",
            ))
        }
    }

    let mut stdout: Vec<u8> = Vec::new();
    let size = child.stdout.unwrap().read_to_end(&mut stdout)?;

    if size > 0 {
        return if let Ok(s) = String::from_utf8(stdout) {
            Ok(s)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid utf8 data in stdout",
            ))
        };
    }

    // if this is reached all if's above did not work
    Err(Error::new(ErrorKind::UnexpectedEof, "keytool stdout empty"))
}
