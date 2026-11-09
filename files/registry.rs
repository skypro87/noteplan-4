//! Windows registry read helpers for Noteplan 4.

pub const APP_KEY: &str = "Software\\Noteplan\\Noteplan 4";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Noteplan 4";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Noteplan 4") },
        RegValue { name: "Version",     value: String::from("v4.3.10") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Noteplan 4") },
        RegValue { name: "Publisher",       value: String::from("Noteplan") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Noteplan 4\\uninstall.exe\"") },
    ]
}
